// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use util::{Address, H160, U256};
use super::account::Account;
use native::storage::*;
use evm::{self, Ext, GasLeft};
use bincode::Infinite;
use bincode::internal::deserialize_from;
use bincode::internal::serialize_into;

pub struct Privacy {
    accounts: Map,        //  address -> balance   H160->U256
    nullifier_set: Array,  // Vec<Nullifier>
    commitments: Map,    // Cmtree 序列化成bytes存储
    output: Vec<u8>,
}

impl Privacy {
    // data[0..4]: func sig;
    // data[4..36]: address, data[36..68]: account balance
    fn set_accounts(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.expect("invalid data");
        let mut pilot = 4;
        let address = H160::from(data.get(pilot + 12 .. pilot + 32)).expect("not enough data");
        pliot += 32;
        let account = data.get(pilot .. pilot + 32).expect("not enough data");
        self.accounts.set(ext, address, account);
        Ok(GasLeft::Known(U256::from(100)))
    }

    // data[4..36]: address, like 00000000000031415926535897932384
    fn get_accounts(&mut self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {
        let data = params.data.expect("invalid data");
        let address = H160::from(data.get(16..36).expect("not enough data"));
        for i in self.accounts.get(ext, address)?.0.iter().rev() {
            serialize_into::<_, _, _, BigEndian>(&mut self.output, &i, Infinite).expect("failed to serialize U256");
        }
        Ok(GasLeft::NeedsReturn(U256::from(100), self.output.as_slice()))
    }




    fn send_remittance(&self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {


    }

    // collection transaction 收款验证
    fn send_collection(&self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {

    }

    // validation 其他节点验证
    fn approve(&self, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error> {

        //加入：验证是否是给自己的汇款

    }



}