# \AddressesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_cities_post**](AddressesApi.md#api1_cities_post) | **POST** /api/1/cities | Cities.
[**api1_regions_post**](AddressesApi.md#api1_regions_post) | **POST** /api/1/regions | Regions.
[**api1_streets_by_city_post**](AddressesApi.md#api1_streets_by_city_post) | **POST** /api/1/streets/by_city | Streets by city.
[**api1_streets_by_id_post**](AddressesApi.md#api1_streets_by_id_post) | **POST** /api/1/streets/by_id | Streets by id or by classifierId.



## api1_cities_post

> models::CitiesResponse api1_cities_post(authorization, timeout, cities_request)
Cities.

   > Restriction group: `Data: geo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**cities_request** | Option<[**CitiesRequest**](CitiesRequest.md)> |  |  |

### Return type

[**models::CitiesResponse**](CitiesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_regions_post

> models::RegionsResponse api1_regions_post(authorization, timeout, regions_request)
Regions.

   > Restriction group: `Data: geo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**regions_request** | Option<[**RegionsRequest**](RegionsRequest.md)> |  |  |

### Return type

[**models::RegionsResponse**](RegionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_streets_by_city_post

> models::StreetsResponse api1_streets_by_city_post(authorization, timeout, streets_by_city_request)
Streets by city.

   > Restriction group: `Data: geo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**streets_by_city_request** | Option<[**StreetsByCityRequest**](StreetsByCityRequest.md)> |  |  |

### Return type

[**models::StreetsResponse**](StreetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_streets_by_id_post

> models::StreetsByIdResponse api1_streets_by_id_post(authorization, timeout, streets_by_id_request)
Streets by id or by classifierId.

   > Restriction group: `Data: geo`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**streets_by_id_request** | Option<[**StreetsByIdRequest**](StreetsByIdRequest.md)> |  |  |

### Return type

[**models::StreetsByIdResponse**](StreetsByIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

