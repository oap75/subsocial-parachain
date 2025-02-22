use frame_support::{
    assert_ok, dispatch::DispatchResult, parameter_types,
    traits::{Currency, Everything},
};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
    testing::Header, traits::{BlakeTwo256, IdentityLookup},
};
use sp_std::convert::{TryInto, TryFrom};

use subsocial_support::Content;
use subsocial_support::mock_functions::{another_valid_content_ipfs, valid_content_ipfs};

pub(crate) use crate as pallet_domains;
use crate::types::*;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
        Timestamp: pallet_timestamp,
        Balances: pallet_balances,
		Domains: pallet_domains,
	}
);

pub(super) type AccountId = u64;
pub(super) type Balance = u64;
type BlockNumber = u64;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

pub(super) const DEFAULT_DOMAIN_DEPOSIT: Balance = 10;

parameter_types! {
    pub static MinDomainLength: u32 = 0;
    pub const MaxDomainLength: u32 = 63;

    pub static MaxDomainsPerAccount: u32 = 0;

    pub const DomainsInsertLimit: u32 = 2860;
    pub static ReservationPeriodLimit: BlockNumber = 0;
    pub const MaxOuterValueLength: u16 = 256;

    pub static BaseDomainDeposit: Balance = DEFAULT_DOMAIN_DEPOSIT;
    pub static OuterValueByteDeposit: Balance = 0;

    pub static InitialPricesConfig: PricesConfigVec<Test> = Vec::new();
    pub const InitialPaymentBeneficiary: AccountId = PAYMENT_BENEFICIARY;
}

impl pallet_domains::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MinDomainLength = MinDomainLength;
    type MaxDomainLength = MaxDomainLength;
    type MaxDomainsPerAccount = MaxDomainsPerAccount;
    type DomainsInsertLimit = DomainsInsertLimit;
    type RegistrationPeriodLimit = ReservationPeriodLimit;
    type MaxOuterValueLength = MaxOuterValueLength;
    type BaseDomainDeposit = BaseDomainDeposit;
    type OuterValueByteDeposit = OuterValueByteDeposit;
    type InitialPaymentBeneficiary = InitialPaymentBeneficiary;
    type InitialPricesConfig = InitialPricesConfig;
    type WeightInfo = ();
}

pub(crate) const DOMAIN_REGISTRAR: u64 = 0;
pub(crate) const DOMAIN_OWNER: u64 = 1;
pub(crate) const DUMMY_ACCOUNT: u64 = 2;
pub(crate) const PAYMENT_BENEFICIARY: u64 = 3;

pub(crate) const ACCOUNT_A: u64 = 10;
pub(crate) const ACCOUNT_B: u64 = 20;

pub(crate) const DEFAULT_TLD: [u8; 3] = *b"sub";

pub(crate) fn origin_a() -> RuntimeOrigin {
    RuntimeOrigin::signed(ACCOUNT_A)
}

pub(crate) fn origin_b() -> RuntimeOrigin {
    RuntimeOrigin::signed(ACCOUNT_B)
}

pub(crate) fn default_tld() -> DomainName<Test> {
    Domains::bound_domain(DEFAULT_TLD.to_vec())
}

pub(crate) fn default_domain() -> DomainName<Test> {
    let tld = default_tld();
    let mut domain_vec = vec![b'A'; MaxDomainLength::get() as usize - tld.len() - 1];

    domain_vec.push(b'.');
    domain_vec.append(&mut tld.to_vec());
    Domains::bound_domain(domain_vec)
}

pub(crate) fn bound_domain_with_default_tld(mut subdomain: Vec<u8>) -> DomainName<Test> {
    subdomain.push(b'.');
    subdomain.append(&mut default_tld().to_vec());
    Domains::bound_domain(subdomain)
}

pub(crate) fn bound_domain_with_custom_tld(mut subdomain: Vec<u8>, mut tld: Vec<u8>) -> DomainName<Test> {
    subdomain.push(b'.');
    subdomain.append(&mut tld);
    Domains::bound_domain(subdomain)
}

pub(crate) fn split_domain_from(string: &[u8]) -> Vec<DomainName<Test>> {
    Domains::split_domain_by_dot(
        &Domains::bound_domain(string.to_vec())
    )
}

pub(crate) fn get_inner_value(domain: &DomainName<Test>) -> Option<InnerValueOf<Test>> {
    Domains::registered_domain(domain).unwrap().inner_value
}

pub(crate) fn get_outer_value(domain: &DomainName<Test>) -> Option<OuterValue<Test>> {
    Domains::registered_domain(domain).unwrap().outer_value
}

pub(crate) fn get_domain_content(domain: &DomainName<Test>) -> Content {
    Domains::registered_domain(domain).unwrap().content
}

pub(crate) fn default_domain_lc() -> DomainName<Test> {
    Domains::lower_domain_then_bound(&default_domain())
}

pub(crate) fn inner_value_account_domain_owner() -> InnerValueOf<Test> {
    InnerValue::Account(DOMAIN_OWNER)
}

pub(crate) fn inner_value_space_id() -> InnerValueOf<Test> {
    InnerValue::Space(1)
}

pub(crate) fn default_outer_value(length: Option<usize>) -> OuterValue<Test> {
    vec![b'A'; length.unwrap_or(MaxOuterValueLength::get() as usize)]
        .try_into().expect("qed; outer value exceeds max length")
}

pub(crate) fn _force_register_domain_with_origin(origin: RuntimeOrigin) -> DispatchResult {
    _register_domain(Some(origin), None, None, None, None)
}

pub(crate) fn _force_register_domain_with_expires_in(expires_in: BlockNumber) -> DispatchResult {
    _register_domain(domain_registrar_origin(), None, None, None, Some(expires_in))
}

pub(crate) fn _force_register_domain_with_name(domain_name: DomainName<Test>) -> DispatchResult {
    _register_domain(domain_registrar_origin(), None, Some(domain_name), None, None)
}

pub(crate) fn _register_default_domain() -> DispatchResult {
    _register_domain(None, None, None, None, None)
}

pub(crate) fn _register_domain(
    origin: Option<RuntimeOrigin>,
    recipient: Option<Option<AccountId>>,
    domain: Option<DomainName<Test>>,
    content: Option<Content>,
    expires_in: Option<BlockNumber>,
) -> DispatchResult {
    Domains::register_domain(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(DOMAIN_OWNER)),
        recipient.unwrap_or_default(),
        domain.unwrap_or_else(default_domain),
        content.unwrap_or_else(valid_content_ipfs),
        expires_in.unwrap_or(ExtBuilder::default().reservation_period_limit),
    )
}

pub(crate) fn _set_inner_value_with_origin(origin: RuntimeOrigin) -> DispatchResult {
    _set_inner_value(Some(origin), None, None)
}

pub(crate) fn _set_default_inner_value() -> DispatchResult {
    _set_inner_value(None, None, None)
}

fn _set_inner_value(
    origin: Option<RuntimeOrigin>,
    domain: Option<DomainName<Test>>,
    value: Option<Option<InnerValueOf<Test>>>,
) -> DispatchResult {
    Domains::set_inner_value(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(DOMAIN_OWNER)),
        domain.unwrap_or_else(default_domain_lc),
        value.unwrap_or_else(|| Some(inner_value_account_domain_owner())),
    )
}

pub(crate) fn _set_outer_value_with_origin(origin: RuntimeOrigin) -> DispatchResult {
    _set_outer_value(Some(origin), None, None)
}

pub(crate) fn _set_outer_value_with_value(value_opt: Option<OuterValue<Test>>) -> DispatchResult {
    _set_outer_value(None, None, Some(value_opt))
}

pub(crate) fn _set_default_outer_value() -> DispatchResult {
    _set_outer_value(None, None, None)
}

fn _set_outer_value(
    origin: Option<RuntimeOrigin>,
    domain: Option<DomainName<Test>>,
    value: Option<Option<OuterValue<Test>>>,
) -> DispatchResult {
    Domains::set_outer_value(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(DOMAIN_OWNER)),
        domain.unwrap_or_else(default_domain_lc),
        value.unwrap_or_else(|| Some(default_outer_value(None))),
    )
}

pub(crate) fn _set_domain_content_with_origin(origin: RuntimeOrigin) -> DispatchResult {
    _set_domain_content(Some(origin), None, None)
}

pub(crate) fn _set_domain_content_with_content(content: Content) -> DispatchResult {
    _set_domain_content(None, None, Some(content))
}

pub(crate) fn _set_default_domain_content() -> DispatchResult {
    _set_domain_content(None, None, None)
}

fn _set_domain_content(
    origin: Option<RuntimeOrigin>,
    domain: Option<DomainName<Test>>,
    content: Option<Content>,
) -> DispatchResult {
    Domains::set_domain_content(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(DOMAIN_OWNER)),
        domain.unwrap_or_else(default_domain_lc),
        content.unwrap_or_else(another_valid_content_ipfs),
    )
}

pub(crate) fn make_free_balance_be(account: &AccountId, balance: Balance) {
    let _ = <Test as pallet_domains::Config>::Currency::make_free_balance_be(account, balance);
}

pub(crate) fn account_with_balance(account: AccountId, balance: Balance) -> AccountId {
    make_free_balance_be(&account, balance);
    account
}

fn domain_registrar_origin() -> Option<RuntimeOrigin> {
    Some(RuntimeOrigin::signed(DOMAIN_REGISTRAR))
}

pub(crate) fn get_reserved_balance(who: &AccountId) -> BalanceOf<Test> {
    <Test as pallet_domains::Config>::Currency::reserved_balance(who)
}

#[derive(Clone)]
pub struct ExtBuilder {
    pub(crate) min_domain_length: u32,
    pub(crate) max_domains_per_account: u32,
    pub(crate) reservation_period_limit: BlockNumber,
    pub(crate) base_domain_deposit: Balance,
    pub(crate) outer_value_byte_deposit: Balance,
    pub(crate) initial_prices_config: PricesConfigVec<Test>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            min_domain_length: 3,
            max_domains_per_account: 10,
            reservation_period_limit: 1000,
            base_domain_deposit: 10,
            outer_value_byte_deposit: 1,
            initial_prices_config: vec![
                (4, 100),
                (5, 50),
                (6, 25),
            ],
        }
    }
}

impl ExtBuilder {
    pub(crate) fn min_domain_length(mut self, min_domain_length: u32) -> Self {
        self.min_domain_length = min_domain_length;
        self
    }

    pub(crate) fn max_domains_per_account(mut self, max_domains_per_account: u32) -> Self {
        self.max_domains_per_account = max_domains_per_account;
        self
    }

    pub(crate) fn reservation_period_limit(mut self, reservation_period_limit: BlockNumber) -> Self {
        self.reservation_period_limit = reservation_period_limit;
        self
    }

    pub(crate) fn base_domain_deposit(mut self, domain_deposit: Balance) -> Self {
        self.base_domain_deposit = domain_deposit;
        self
    }

    pub(crate) fn outer_value_byte_deposit(mut self, outer_value_byte_deposit: Balance) -> Self {
        self.outer_value_byte_deposit = outer_value_byte_deposit;
        self
    }

    pub(crate) fn initial_prices(mut self, initial_prices: PricesConfigVec<Test>) -> Self {
        self.initial_prices_config = initial_prices;
        self
    }

    fn set_configs(&self) {
        MIN_DOMAIN_LENGTH.with(|x| *x.borrow_mut() = self.min_domain_length);
        MAX_DOMAINS_PER_ACCOUNT.with(|x| *x.borrow_mut() = self.max_domains_per_account);
        BASE_DOMAIN_DEPOSIT.with(|x| *x.borrow_mut() = self.base_domain_deposit);
        OUTER_VALUE_BYTE_DEPOSIT.with(|x| *x.borrow_mut() = self.outer_value_byte_deposit);
        RESERVATION_PERIOD_LIMIT.with(|x| *x.borrow_mut() = self.reservation_period_limit);
        INITIAL_PRICES_CONFIG.with(|x| *x.borrow_mut() = self.initial_prices_config.clone());
    }

    pub(crate) fn build(self) -> TestExternalities {
        self.set_configs();

        let storage = &mut frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = TestExternalities::from(storage.clone());
        ext.execute_with(|| {
            System::set_block_number(1);
            assert_ok!(
                Domains::support_tlds(
                    RuntimeOrigin::root(),
                    vec![default_tld()].try_into().expect("qed; domains vector exceeds the limit"),
                )
            );
        });

        ext
    }

    pub(crate) fn build_with_default_domain_registered(self) -> TestExternalities {
        let mut ext = self.clone().build();
        ext.execute_with(|| {
            let subdomain = pallet_domains::Pallet::<Test>::split_domain_by_dot(&default_domain_lc())
                .first()
                .cloned()
                .unwrap();

            let domain_price = pallet_domains::Pallet::<Test>::calculate_price(&subdomain).unwrap();

            let _ = account_with_balance(DOMAIN_OWNER, self.base_domain_deposit + domain_price);
            assert_ok!(_register_default_domain());
        });
        ext
    }
}
