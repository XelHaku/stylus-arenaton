// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

interface IArenatonEngine {
    struct PlayerSummary {
        uint256 level;
        uint256 ethBalance;
        uint256 atonBalance;
        uint256 unclaimedCommission;
        uint256 claimedCommission;
    }

    struct EventDTO {
        string eventId;
        uint256 startDate;
        uint8 sport;
        uint256 total_A;
        uint256 total_B;
        uint256 total;
        int8 winner;
        Stake playerStake;
        bool active;
        bool closed;
        bool paid;
    }

    struct Stake {
        uint256 amount;
        uint8 team;
    }

    enum Step {
        Opened,
        Closed,
        Paid
    }

    // Event management
    function addEvent(
        string memory _eventId,
        uint256 _startDate,
        uint8 _sport
    ) external;

    function terminateEvent(
        string memory eventId,
        int8 _winner,
        uint8 _batchSize
    ) external;

    function getEventDTO(
        address _player,
        string memory _eventId
    ) external view returns (EventDTO memory);

    function getEvents(
        uint8 _sport,
        Step _step,
        address _player
    ) external view returns (EventDTO[] memory);

    // Player management
    function playerSummary(address playerAddress)
        external
        view
        returns (
            PlayerSummary memory summary,
            uint256 totalCommission,
            uint256 accumulatedCommission
        );

    function getPlayerEvents(
        address playerAddress,
        uint8 sport,
        bool active,
        uint256 size,
        uint256 pageNo
    ) external view returns (EventDTO[] memory);

    // ATON Integration
    function donateATON() external payable;

    function swap(uint256 _amountAton) external returns (bool success);
}
