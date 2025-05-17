# \DeprecatedApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_deliveries_check_create_post**](DeprecatedApi.md#api1_deliveries_check_create_post) | **POST** /api/1/deliveries/check_create | Check create delivery.
[**api1_deliveries_update_order_payments_post**](DeprecatedApi.md#api1_deliveries_update_order_payments_post) | **POST** /api/1/deliveries/update_order_payments | Update order payment details.
[**api1_organizations_get**](DeprecatedApi.md#api1_organizations_get) | **GET** /api/1/organizations | Returns organizations available to api-login user.



## api1_deliveries_check_create_post

> models::CheckCreateOrderResponse api1_deliveries_check_create_post(authorization, timeout, request_create_order_request)
Check create delivery.

> Deprecated, all checks are available in `api/1/deliveries/create`.   > Restriction group: `Deprecated`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_create_order_request** | Option<[**RequestCreateOrderRequest**](RequestCreateOrderRequest.md)> |  |  |

### Return type

[**models::CheckCreateOrderResponse**](CheckCreateOrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_update_order_payments_post

> models::CorrelationIdResponse api1_deliveries_update_order_payments_post(authorization, timeout, request_update_order_payments_request)
Update order payment details.

> Deprecated, use `api/1/deliveries/change_payments` method instead.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Deprecated`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_update_order_payments_request** | Option<[**RequestUpdateOrderPaymentsRequest**](RequestUpdateOrderPaymentsRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_organizations_get

> models::OrderGetSimpleOrganizationsResponse api1_organizations_get(authorization, timeout)
Returns organizations available to api-login user.

> Deprecated, use `POST api/1/organizations`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]

### Return type

[**models::OrderGetSimpleOrganizationsResponse**](OrderGetSimpleOrganizationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

