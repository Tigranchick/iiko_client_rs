# \CustomerCategoriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_loyalty_iiko_customer_category_add_post**](CustomerCategoriesApi.md#api1_loyalty_iiko_customer_category_add_post) | **POST** /api/1/loyalty/iiko/customer_category/add | Add category for customer.
[**api1_loyalty_iiko_customer_category_post**](CustomerCategoriesApi.md#api1_loyalty_iiko_customer_category_post) | **POST** /api/1/loyalty/iiko/customer_category | Get customer categories.
[**api1_loyalty_iiko_customer_category_remove_post**](CustomerCategoriesApi.md#api1_loyalty_iiko_customer_category_remove_post) | **POST** /api/1/loyalty/iiko/customer_category/remove | Remove category for customer.



## api1_loyalty_iiko_customer_category_add_post

> serde_json::Value api1_loyalty_iiko_customer_category_add_post(authorization, timeout, change_category_for_customer_request)
Add category for customer.

Add specified category for customer.   > Restriction group: `Guests: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**change_category_for_customer_request** | Option<[**ChangeCategoryForCustomerRequest**](ChangeCategoryForCustomerRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_category_post

> models::GetCategoriesResponse api1_loyalty_iiko_customer_category_post(authorization, timeout, get_categories_request)
Get customer categories.

Get all organization's customer categories.   > Restriction group: `Loyalty: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_categories_request** | Option<[**GetCategoriesRequest**](GetCategoriesRequest.md)> |  |  |

### Return type

[**models::GetCategoriesResponse**](GetCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_category_remove_post

> serde_json::Value api1_loyalty_iiko_customer_category_remove_post(authorization, timeout, change_category_for_customer_request)
Remove category for customer.

Remove specified category for customer.   > Restriction group: `Guests: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**change_category_for_customer_request** | Option<[**ChangeCategoryForCustomerRequest**](ChangeCategoryForCustomerRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

