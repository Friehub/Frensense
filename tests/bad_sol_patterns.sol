pragma solidity ^0.8.0;

contract Vault {
    address public owner;
    uint256 public balance;

    // 1. Missing Access Control but weirdly formatted
    function 
    withdrawAll() 
    external 
    {
        // SOL_MISSING_ACCESS_CONTROL
        payable(msg.sender).transfer(address(this).balance);
    }

    // 2. Safe function but missing only owner (uses require instead)
    function withdrawSafe(uint256 amount) public {
        require(msg.sender == owner, "Not owner"); // Should NOT trigger SOL_MISSING_ACCESS_CONTROL
        payable(msg.sender).transfer(amount);
    }

    // 3. Unchecked block
    function unsafeMath(uint256 x) public pure returns (uint256) {
        unchecked {
            return x - 1; // SOL_INTEGER_OVERFLOW
        }
    }
}
