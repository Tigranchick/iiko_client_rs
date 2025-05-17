# \TerminalGroupsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_terminal_groups_awake_post**](TerminalGroupsApi.md#api1_terminal_groups_awake_post) | **POST** /api/1/terminal_groups/awake | Awake terminal groups from sleep mode.
[**api1_terminal_groups_is_alive_post**](TerminalGroupsApi.md#api1_terminal_groups_is_alive_post) | **POST** /api/1/terminal_groups/is_alive | Returns information on availability of group of terminals.
[**api1_terminal_groups_post**](TerminalGroupsApi.md#api1_terminal_groups_post) | **POST** /api/1/terminal_groups | Method that returns information on groups of delivery terminals.



## api1_terminal_groups_awake_post

> models::AwakeTerminalGroupsResponse api1_terminal_groups_awake_post(authorization, timeout, awake_terminal_groups_request)
Awake terminal groups from sleep mode.

   > Restriction group: `Organizations: settings`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**awake_terminal_groups_request** | Option<[**AwakeTerminalGroupsRequest**](AwakeTerminalGroupsRequest.md)> |  |  |

### Return type

[**models::AwakeTerminalGroupsResponse**](AwakeTerminalGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_terminal_groups_is_alive_post

> models::TerminalGroupsIsAliveResponse api1_terminal_groups_is_alive_post(authorization, timeout, terminal_groups_is_alive_request)
Returns information on availability of group of terminals.

   > Restriction group: `POS: availability`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**terminal_groups_is_alive_request** | Option<[**TerminalGroupsIsAliveRequest**](TerminalGroupsIsAliveRequest.md)> |  |  |

### Return type

[**models::TerminalGroupsIsAliveResponse**](TerminalGroupsIsAliveResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_terminal_groups_post

> models::TerminalGroupsResponse api1_terminal_groups_post(authorization, timeout, terminal_groups_request)
Method that returns information on groups of delivery terminals.

   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**terminal_groups_request** | Option<[**TerminalGroupsRequest**](TerminalGroupsRequest.md)> |  |  |

### Return type

[**models::TerminalGroupsResponse**](TerminalGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

