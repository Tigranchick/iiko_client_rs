# \OperationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_commands_status_post**](OperationsApi.md#api1_commands_status_post) | **POST** /api/1/commands/status | Get status of command.



## api1_commands_status_post

> models::GetCommandStatusResponse api1_commands_status_post(authorization, timeout, get_command_status_request)
Get status of command.

> Response code `410` means that the correlationId value specified in the method is no longer supported.  Please do not request methods that include such a value.   > Restriction group: `Commands`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_command_status_request** | Option<[**GetCommandStatusRequest**](GetCommandStatusRequest.md)> |  |  |

### Return type

[**models::GetCommandStatusResponse**](GetCommandStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

