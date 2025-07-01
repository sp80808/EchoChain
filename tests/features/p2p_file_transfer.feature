Feature: P2P File Transfer
  As a content creator
  I want to share files via the P2P network
  So I can distribute my content efficiently

  Background:
    Given I have a file "sample.mp3" to share
    And I am connected to the P2P network

  Scenario: Discover available files
    Given there are files available on the network
    When I search for files matching "sample"
    Then I should see a list of matching files
    And each result should show:
      | Field       | Description            |
      | name        | File name              |
      | size        | File size in bytes     |
      | type        | File type              |
      | owner       | Owner's public key     |
      | chunks      | Available chunks count |

  Scenario: Initiate file transfer
    Given I have selected a file to download
    When I initiate the download
    Then the transfer should start within 5 seconds
    And I should receive progress updates
    And the file should be saved to my downloads folder when complete

  Scenario: Verify file integrity
    Given I have downloaded a file
    When I verify the file hash
    Then it should match the original file's hash
    And all chunks should be validated

  Scenario: Resume interrupted transfer
    Given a previous transfer was interrupted at 50%
    When I resume the transfer
    Then it should continue from the last received chunk
    And the progress should reflect previously downloaded chunks

  Scenario: Share a new file
    Given I have a new file to share
    When I register it with the P2P network
    Then the file should be chunked and indexed
    And other peers should be able to discover it
    And I should receive requests for chunks