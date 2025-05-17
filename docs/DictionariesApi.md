# \DictionariesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_cancel_causes_post**](DictionariesApi.md#api1_cancel_causes_post) | **POST** /api/1/cancel_causes | Delivery cancel causes.
[**api1_deliveries_order_types_post**](DictionariesApi.md#api1_deliveries_order_types_post) | **POST** /api/1/deliveries/order_types | Order types.
[**api1_discounts_post**](DictionariesApi.md#api1_discounts_post) | **POST** /api/1/discounts | Discounts / surcharges.
[**api1_payment_types_post**](DictionariesApi.md#api1_payment_types_post) | **POST** /api/1/payment_types | Payment types.
[**api1_removal_types_post**](DictionariesApi.md#api1_removal_types_post) | **POST** /api/1/removal_types | Removal types (reasons for deletion).
[**api1_tips_types_post**](DictionariesApi.md#api1_tips_types_post) | **POST** /api/1/tips_types | Get tips types for api-login`s rms group.



## api1_cancel_causes_post

> models::CancelCausesResponse api1_cancel_causes_post(authorization, timeout, cancel_causes_request)
Delivery cancel causes.

   > Allowed from version `7.7.1`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**cancel_causes_request** | Option<[**CancelCausesRequest**](CancelCausesRequest.md)> |  |  |

### Return type

[**models::CancelCausesResponse**](CancelCausesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_order_types_post

> models::OrderTypesResponse api1_deliveries_order_types_post(authorization, timeout, order_types_request)
Order types.

   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**order_types_request** | Option<[**OrderTypesRequest**](OrderTypesRequest.md)> |  |  |

### Return type

[**models::OrderTypesResponse**](OrderTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_discounts_post

> models::DiscountsResponse api1_discounts_post(authorization, timeout, discounts_request)
Discounts / surcharges.

   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**discounts_request** | Option<[**DiscountsRequest**](DiscountsRequest.md)> |  |  |

### Return type

[**models::DiscountsResponse**](DiscountsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_payment_types_post

> models::PaymentTypesResponse api1_payment_types_post(authorization, timeout, payment_types_request)
Payment types.

   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**payment_types_request** | Option<[**PaymentTypesRequest**](PaymentTypesRequest.md)> |  |  |

### Return type

[**models::PaymentTypesResponse**](PaymentTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_removal_types_post

> models::RemovalTypesResponse api1_removal_types_post(authorization, timeout, removal_types_request)
Removal types (reasons for deletion).

   > Allowed from version `7.5.3`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**removal_types_request** | Option<[**RemovalTypesRequest**](RemovalTypesRequest.md)> |  |  |

### Return type

[**models::RemovalTypesResponse**](RemovalTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_tips_types_post

> models::TipsTypesResponse api1_tips_types_post(authorization, timeout)
Get tips types for api-login`s rms group.

   > Allowed from version `7.7.4`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]

### Return type

[**models::TipsTypesResponse**](TipsTypesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

