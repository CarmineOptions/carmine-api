
import argparse
import json
import time
from dataclasses import dataclass
import pickle
import asyncio
from typing import List, Optional, Any

from starknet_py.net.gateway_client import GatewayClient
from starknet_py.hash.selector import get_selector_from_name
from starknet_py.net.client_models import Call


AMM_ADDR = 0x076dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa  # mainnet
NET = GatewayClient(net="mainnet")


@dataclass
class Option:
    option_side: int
    maturity: int
    strike_price: float
    quote_token_address: float
    base_token_address: float
    option_type: int
    pool_position: Optional[float]
    volatility: Optional[float]


@dataclass
class LiqPool:
    address: int
    option_type: int
    unlocked_capital: float
    locked_capital: float
    lpool_bal: float
    value_pool_position: float
    options: List[Option]


@dataclass
class AmmState:
    pools: List[LiqPool]
    time: int


class AMM:
    def __init__(self, net: GatewayClient, addr: int):
        self.net = net
        self.addr = addr
        self.pools = {}

    async def func_call(self, selector: str, calldata: List[Any], note: Optional[str] = None):
        call = Call(
            to_addr=self.addr,
            selector=get_selector_from_name(selector),
            calldata=calldata
        )
        res = await self.net.call_contract(call)
        return (calldata, res) if note is None else (calldata, res, note)

    async def get_liq_pools_with_options(self):
        all_lptokens_call = Call(
            to_addr=self.addr,
            selector=get_selector_from_name('get_all_lptoken_addresses'),
            calldata=[],
        )
        res = await self.net.call_contract(all_lptokens_call)
        lp_addrs = res[1:]

        pool_tasks = [
            asyncio.create_task(self.func_call('get_all_options', [lp_addr])) for lp_addr in lp_addrs
        ]

        pool_defs = await asyncio.gather(*pool_tasks)

        pools = {}
        for calldata, res in pool_defs:
            pools[calldata[0]] = {}
            opts = []

            for i in range(1, len(res), 6):
                opts.append(
                    {
                        'option_side': res[i],
                        'maturity': res[i + 1],
                        'strike_price': res[i + 2],
                        'quote_token_address': res[i + 3],
                        'base_token_address': res[i + 4],
                        'option_type': res[i + 5],
                        'pool_position':  None,
                        'volatility':  None
                    }
                )

            pools[calldata[0]]['options'] = opts

        self.pools = pools

    async def get_locked_unlocked_total_capital(self):

        pools = self.pools.keys()

        locked_tasks = [
            asyncio.create_task(self.func_call('get_pool_locked_capital', [pool_adr], 'locked_cap')) for pool_adr in pools
        ]
        unlocked_tasks = [
            asyncio.create_task(self.func_call('get_unlocked_capital', [pool_adr], 'unlocked_cap')) for pool_adr in pools
        ]
        total_tasks = [
            asyncio.create_task(self.func_call('get_lpool_balance', [pool_adr], 'lpool_balance')) for pool_adr in pools
        ]

        locked = await asyncio.gather(*locked_tasks)
        unlocked = await asyncio.gather(*unlocked_tasks)
        total = await asyncio.gather(*total_tasks)

        for calldata, res, note in unlocked:
            self.pools[calldata[0]][note] = res[0]
        for calldata, res, note in locked:
            self.pools[calldata[0]][note] = res[0]
        for calldata, res, note in total:
            self.pools[calldata[0]][note] = res[0]

    async def get_pool_positions(self):

        pool_pos_tasks = [
            asyncio.create_task(self.func_call('get_value_of_pool_position', [pool_adr], None)) for pool_adr in self.pools.keys()
        ]

        pool_pos = await asyncio.gather(*pool_pos_tasks)

        for calldata, res in pool_pos:
            self.pools[calldata[0]]['value_pool_position'] = res[0]

    async def get_option_positions(self):
        for key, value in self.pools.items():
            pos_tasks = []
            for option in value['options']:
                pos_tasks.append(
                    self.func_call(
                        selector='get_option_position',
                        calldata=[key, option['option_side'],
                                  option['maturity'], option['strike_price']],
                        note=None
                    )
                )
            opt_pos = await asyncio.gather(*pos_tasks)

            for (_, side, maturity, strike), res in opt_pos:
                for option in value['options']:
                    if option['option_side'] == side:
                        if option['maturity'] == maturity:
                            if option['strike_price'] == strike:
                                option['pool_position'] = res[0]

    async def get_volatilities(self):
        for key, value in self.pools.items():
            vol_tasks = []
            for option in value['options']:
                vol_tasks.append(
                    self.func_call(
                        selector='get_pool_volatility_auto',
                        calldata=[key, option['maturity'],
                                  option['strike_price']],
                        note=None
                    )
                )
            volatities = await asyncio.gather(*vol_tasks)

            for (_, maturity, strike), res in volatities:
                for option in value['options']:
                    if option['maturity'] == maturity:
                        if option['strike_price'] == strike:
                            option['volatility'] = res[0]

    async def run(self):
        await self.get_liq_pools_with_options()
        await self.get_locked_unlocked_total_capital()
        await self.get_pool_positions()
        await self.get_option_positions()
        await self.get_volatilities()

    async def get_state(self) -> AmmState:
        await self.run()

        json_state = json.dumps({
            "pools": self.pools,
            "time": round(time.time())
        }, indent=4)

        pools = []
        for pool, values in self.pools.items():
            for opt in values['options']:
                opt['strike_price'] = opt['strike_price'] / 2**61
                opt['pool_position'] = opt['pool_position'] / 10**18
                opt['volatility'] = opt['volatility'] / 2**61

            _type = values['options'][0]['option_type']
            pools.append(
                LiqPool(
                    address=pool,
                    option_type=values['options'][0]['option_type'],
                    unlocked_capital=values['unlocked_cap'] /
                    10**18 if _type == 0 else values['unlocked_cap'] / 10**6,
                    locked_capital=values['locked_cap'] /
                    10**18 if _type == 0 else values['locked_cap'] / 10**6,
                    lpool_bal=values['lpool_balance'] /
                    10**18 if _type == 0 else values['lpool_balance'] / 10**6,
                    value_pool_position=values['value_pool_position'] / 2**61,
                    options=[
                        Option(
                            option_side=option['option_side'],
                            maturity=option['maturity'],
                            strike_price=option['strike_price'],
                            quote_token_address=option['quote_token_address'],
                            base_token_address=option['base_token_address'],
                            option_type=option['option_type'],
                            pool_position=option['pool_position'],
                            volatility=option['volatility']
                        ) for option in values['options']
                    ]
                )
            )
        return (AmmState(pools=pools, time=round(time.time())), json_state)


async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--type", help="Data type [json, dataclass]")
    parser.add_argument("-o", "--output", help="Output type [print, file]")
    args = parser.parse_args()
    type = args.type or "dataclass"
    output = args.output or "file"

    (amm_state, json_state) = await AMM(NET, AMM_ADDR).get_state()

    if output == "print":
        print(json_state) if type == "json" else print(amm_state)
    else:
        save_path = "amm_state.json" if type == "json" else "amm_state.pickle"
        if type == "json":
            with open(save_path, 'w') as infile:
                infile.write(json_state)
                infile.close()
        else:
            with open(save_path, 'wb') as infile:
                pickle.dump(amm_state, infile)


if __name__ == '__main__':
    asyncio.run(main())
