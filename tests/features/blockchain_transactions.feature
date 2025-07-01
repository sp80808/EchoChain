Feature: Blockchain Transactions
  As a user of the EchoChain application
  I want to interact with the blockchain
  So I can register content and transfer tokens

  Background:
    Given I have a valid wallet with balance
    And I am connected to the EchoChain network

  Scenario: Transfer tokens between accounts
    Given I have a recipient address "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    When I transfer "10 ECHO" to the recipient
    Then the transaction should be confirmed within 60 seconds
    And my balance should decrease by "10 ECHO"
    And the recipient's balance should increase by "10 ECHO"

  Scenario: Register content metadata on-chain
    Given I have a content file "sample.mp3" with metadata
      | title       | My Sample Track |
      | artist      | Test Artist     |
      | duration    | 180             |
      | sample_rate | 44100           |
    When I register the content metadata on-chain
    Then the transaction should be confirmed within 60 seconds
    And the content should be assigned a unique content ID
    And the metadata should be retrievable from the chain

  Scenario: Claim content contribution rewards
    Given I have contributed content that has been used
    And the reward period has ended
    When I claim my contribution rewards
    Then the transaction should be confirmed within 60 seconds
    And my balance should increase by the reward amount
    And my contribution record should be updated

  Scenario: View transaction history
    Given I have performed multiple transactions
    When I view my transaction history
    Then I should see all my transactions ordered by time
    And each transaction should show:
      | Field       | Description                  |
      | hash        | Transaction hash             |
      | block       | Block number                 |
      | timestamp   | Time of confirmation         |
      | type        | Transaction type             |
      | amount      | Amount transferred (if any)  |
      | status      | Confirmation status          |