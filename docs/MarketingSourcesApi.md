# \MarketingSourcesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_marketing_sources_post**](MarketingSourcesApi.md#api1_marketing_sources_post) | **POST** /api/1/marketing_sources | Marketing sources.



## api1_marketing_sources_post

> models::MarketingSourcesResponse api1_marketing_sources_post(authorization, timeout, marketing_sources_request)
Marketing sources.

   > Allowed from version `7.2.5`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**marketing_sources_request** | Option<[**MarketingSourcesRequest**](MarketingSourcesRequest.md)> |  |  |

### Return type

[**models::MarketingSourcesResponse**](MarketingSourcesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

