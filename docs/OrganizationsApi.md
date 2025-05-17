# \OrganizationsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_organizations_post**](OrganizationsApi.md#api1_organizations_post) | **POST** /api/1/organizations | Returns organizations available to api-login user.
[**api1_organizations_settings_post**](OrganizationsApi.md#api1_organizations_settings_post) | **POST** /api/1/organizations/settings | Returns available to api-login user organizations specified settings.



## api1_organizations_post

> models::OrderGetOrganizationsResponse api1_organizations_post(authorization, timeout, order_get_organizations_request)
Returns organizations available to api-login user.

   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**order_get_organizations_request** | Option<[**OrderGetOrganizationsRequest**](OrderGetOrganizationsRequest.md)> |  |  |

### Return type

[**models::OrderGetOrganizationsResponse**](OrderGetOrganizationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_organizations_settings_post

> models::OrderOrganizationsSettingsResponse api1_organizations_settings_post(authorization, timeout, order_organizations_settings_request)
Returns available to api-login user organizations specified settings.

   > Restriction group: `Organizations: settings`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**order_organizations_settings_request** | Option<[**OrderOrganizationsSettingsRequest**](OrderOrganizationsSettingsRequest.md)> |  |  |

### Return type

[**models::OrderOrganizationsSettingsResponse**](OrderOrganizationsSettingsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

