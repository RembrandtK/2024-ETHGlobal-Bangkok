use ethers::contract::abigen;

abigen!(
    Contract,
    r#"[
        function number() external view returns (uint256)
        function setNumber(uint256 number) external
        function increment() external
    ]"#
);
