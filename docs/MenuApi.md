# \MenuApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_combo_calculate_post**](MenuApi.md#api1_combo_calculate_post) | **POST** /api/1/combo/calculate | Calculate combo price
[**api1_combo_post**](MenuApi.md#api1_combo_post) | **POST** /api/1/combo | Get combos info
[**api1_nomenclature_post**](MenuApi.md#api1_nomenclature_post) | **POST** /api/1/nomenclature | Menu.
[**api1_stop_lists_add_post**](MenuApi.md#api1_stop_lists_add_post) | **POST** /api/1/stop_lists/add | Add items to out-of-stock list.  (You should have extra rights to use this method).
[**api1_stop_lists_check_post**](MenuApi.md#api1_stop_lists_check_post) | **POST** /api/1/stop_lists/check | Check items in out-of-stock list.
[**api1_stop_lists_clear_post**](MenuApi.md#api1_stop_lists_clear_post) | **POST** /api/1/stop_lists/clear | Clear out-of-stock list.  (You should have extra rights to use this method).
[**api1_stop_lists_post**](MenuApi.md#api1_stop_lists_post) | **POST** /api/1/stop_lists | Out-of-stock items.
[**api1_stop_lists_remove_post**](MenuApi.md#api1_stop_lists_remove_post) | **POST** /api/1/stop_lists/remove | Remove items from out-of-stock list.  (You should have extra rights to use this method).
[**api2_menu_by_id_post**](MenuApi.md#api2_menu_by_id_post) | **POST** /api/2/menu/by_id | Retrieve external menu by ID.
[**api2_menu_post**](MenuApi.md#api2_menu_post) | **POST** /api/2/menu | External menus with price categories.



## api1_combo_calculate_post

> models::CalculateComboPriceResponse api1_combo_calculate_post(authorization, timeout, calculate_combo_price_request)
Calculate combo price

Make combo price calculation.   > Restriction group: `Loyalty: order calculate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**calculate_combo_price_request** | Option<[**CalculateComboPriceRequest**](CalculateComboPriceRequest.md)> |  |  |

### Return type

[**models::CalculateComboPriceResponse**](CalculateComboPriceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_combo_post

> models::GetCombosInfoResponse api1_combo_post(authorization, timeout, get_combos_info_request)
Get combos info

Get all organization's combos.   > Restriction group: `Data: menu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_combos_info_request** | Option<[**GetCombosInfoRequest**](GetCombosInfoRequest.md)> |  |  |

### Return type

[**models::GetCombosInfoResponse**](GetCombosInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_nomenclature_post

> models::NomenclatureResponse api1_nomenclature_post(authorization, timeout, nomenclature_request)
Menu.

> Sourced from RMS Data Exchange Export menu.   > Restriction group: `Data: menu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**nomenclature_request** | Option<[**NomenclatureRequest**](NomenclatureRequest.md)> |  |  |

### Return type

[**models::NomenclatureResponse**](NomenclatureResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_stop_lists_add_post

> models::CorrelationIdResponse api1_stop_lists_add_post(authorization, timeout, add_products_to_stop_list_request)
Add items to out-of-stock list.  (You should have extra rights to use this method).

   > Allowed from version `8.6.1`.   > Restriction group: `Data: changing stoplists`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_products_to_stop_list_request** | Option<[**AddProductsToStopListRequest**](AddProductsToStopListRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_stop_lists_check_post

> models::CheckStopListResponse api1_stop_lists_check_post(authorization, timeout, check_stop_list_request)
Check items in out-of-stock list.

   > Restriction group: `Orders: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**check_stop_list_request** | Option<[**CheckStopListRequest**](CheckStopListRequest.md)> |  |  |

### Return type

[**models::CheckStopListResponse**](CheckStopListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_stop_lists_clear_post

> models::CorrelationIdResponse api1_stop_lists_clear_post(authorization, timeout, clear_stop_list_request)
Clear out-of-stock list.  (You should have extra rights to use this method).

   > Allowed from version `8.6.1`.   > Restriction group: `Data: changing stoplists`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**clear_stop_list_request** | Option<[**ClearStopListRequest**](ClearStopListRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_stop_lists_post

> models::StopListsResponse api1_stop_lists_post(authorization, timeout, stop_lists_request)
Out-of-stock items.

   > Restriction group: `Data: stoplists`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**stop_lists_request** | Option<[**StopListsRequest**](StopListsRequest.md)> |  |  |

### Return type

[**models::StopListsResponse**](StopListsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_stop_lists_remove_post

> models::CorrelationIdResponse api1_stop_lists_remove_post(authorization, timeout, remove_products_from_stop_list_request)
Remove items from out-of-stock list.  (You should have extra rights to use this method).

   > Allowed from version `8.6.1`.   > Restriction group: `Data: changing stoplists`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**remove_products_from_stop_list_request** | Option<[**RemoveProductsFromStopListRequest**](RemoveProductsFromStopListRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api2_menu_by_id_post

> models::ExternalMenuPreset api2_menu_by_id_post(authorization, timeout, menu_request)
Retrieve external menu by ID.

> Sourced from Web External menu.   > Restriction group: `Data: menu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**menu_request** | Option<[**MenuRequest**](MenuRequest.md)> |  |  |

### Return type

[**models::ExternalMenuPreset**](ExternalMenuPreset.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api2_menu_post

> models::MenusDataResponse api2_menu_post(authorization, timeout)
External menus with price categories.

   > Restriction group: `Data: menu`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]

### Return type

[**models::MenusDataResponse**](MenusDataResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

