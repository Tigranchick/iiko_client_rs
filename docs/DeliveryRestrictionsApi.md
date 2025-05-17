# \DeliveryRestrictionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_delivery_restrictions_allowed_post**](DeliveryRestrictionsApi.md#api1_delivery_restrictions_allowed_post) | **POST** /api/1/delivery_restrictions/allowed | Get suitable terminal groups for delivery restrictions.
[**api1_delivery_restrictions_post**](DeliveryRestrictionsApi.md#api1_delivery_restrictions_post) | **POST** /api/1/delivery_restrictions | Retrieve list of delivery restrictions.



## api1_delivery_restrictions_allowed_post

> models::GetAllowedRestrictionsResponse api1_delivery_restrictions_allowed_post(authorization, timeout, get_allowed_restrictions_request)
Get suitable terminal groups for delivery restrictions.

   > Allowed from version `6.4.16`.   > Restriction group: `Orders: preparing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_allowed_restrictions_request** | Option<[**GetAllowedRestrictionsRequest**](GetAllowedRestrictionsRequest.md)> |  |  |

### Return type

[**models::GetAllowedRestrictionsResponse**](GetAllowedRestrictionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_delivery_restrictions_post

> models::GetDeliveryRestrictionsResponse api1_delivery_restrictions_post(authorization, timeout, get_delivery_restrictions_request)
Retrieve list of delivery restrictions.

   > Allowed from version `6.4.16`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_delivery_restrictions_request** | Option<[**GetDeliveryRestrictionsRequest**](GetDeliveryRestrictionsRequest.md)> |  |  |

### Return type

[**models::GetDeliveryRestrictionsResponse**](GetDeliveryRestrictionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

