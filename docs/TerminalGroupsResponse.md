# TerminalGroupsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Operation ID. | 
**terminal_groups** | [**Vec<models::RmsTerminalGroup>**](RmsTerminalGroup.md) | List of terminal groups broken down by organizations. | 
**terminal_groups_in_sleep** | [**Vec<models::RmsTerminalGroup>**](RmsTerminalGroup.md) | Terminal groups are in sleep mode because they are not active.    Can be awakened by `/api/1/terminal_groups/awake` operation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


