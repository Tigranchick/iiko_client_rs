# \AuthorizationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_access_token_post**](AuthorizationApi.md#api1_access_token_post) | **POST** /api/1/access_token | Retrieve session key for API user.



## api1_access_token_post

> models::GetAccessTokenResponse api1_access_token_post(timeout, get_access_token_request)
Retrieve session key for API user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_access_token_request** | Option<[**GetAccessTokenRequest**](GetAccessTokenRequest.md)> |  |  |

### Return type

[**models::GetAccessTokenResponse**](GetAccessTokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

