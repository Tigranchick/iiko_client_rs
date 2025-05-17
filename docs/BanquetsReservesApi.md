# \BanquetsReservesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_reserve_add_items_post**](BanquetsReservesApi.md#api1_reserve_add_items_post) | **POST** /api/1/reserve/add_items | Add order items.
[**api1_reserve_add_payments_post**](BanquetsReservesApi.md#api1_reserve_add_payments_post) | **POST** /api/1/reserve/add_payments | Add order payments.
[**api1_reserve_available_organizations_post**](BanquetsReservesApi.md#api1_reserve_available_organizations_post) | **POST** /api/1/reserve/available_organizations | Returns all organizations of current account (determined by Authorization request header) for which banquet/reserve booking are available.
[**api1_reserve_available_restaurant_sections_post**](BanquetsReservesApi.md#api1_reserve_available_restaurant_sections_post) | **POST** /api/1/reserve/available_restaurant_sections | Returns all restaurant sections of specified terminal groups, for which banquet/reserve booking are available.
[**api1_reserve_available_terminal_groups_post**](BanquetsReservesApi.md#api1_reserve_available_terminal_groups_post) | **POST** /api/1/reserve/available_terminal_groups | Returns all terminal groups of specified organizations, for which banquet/reserve booking are available.
[**api1_reserve_cancel_post**](BanquetsReservesApi.md#api1_reserve_cancel_post) | **POST** /api/1/reserve/cancel | Cancel reservation due to some reason.
[**api1_reserve_create_post**](BanquetsReservesApi.md#api1_reserve_create_post) | **POST** /api/1/reserve/create | Create banquet/reserve.
[**api1_reserve_restaurant_sections_workload_post**](BanquetsReservesApi.md#api1_reserve_restaurant_sections_workload_post) | **POST** /api/1/reserve/restaurant_sections_workload | Returns all banquets/reserves for passed restaurant sections.
[**api1_reserve_status_by_id_post**](BanquetsReservesApi.md#api1_reserve_status_by_id_post) | **POST** /api/1/reserve/status_by_id | Retrieve banquets/reserves statuses by IDs.



## api1_reserve_add_items_post

> models::CorrelationIdResponse api1_reserve_add_items_post(authorization, timeout, add_order_items_to_banquet_request)
Add order items.

Available only for banquets.   > Allowed from version `8.2.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_order_items_to_banquet_request** | Option<[**AddOrderItemsToBanquetRequest**](AddOrderItemsToBanquetRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_add_payments_post

> models::CorrelationIdResponse api1_reserve_add_payments_post(authorization, timeout, add_order_payments_to_banquet_request)
Add order payments.

Available only for banquets.   > Allowed from version `8.2.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order payments: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_order_payments_to_banquet_request** | Option<[**AddOrderPaymentsToBanquetRequest**](AddOrderPaymentsToBanquetRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_available_organizations_post

> models::OrderGetOrganizationsResponse api1_reserve_available_organizations_post(authorization, timeout, order_get_organizations_request)
Returns all organizations of current account (determined by Authorization request header) for which banquet/reserve booking are available.

   > Allowed from version `7.1.5`.   > Restriction group: `Orders: preparing`.

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


## api1_reserve_available_restaurant_sections_post

> models::GetRestaurantSectionsResponse api1_reserve_available_restaurant_sections_post(authorization, timeout, get_restaurant_sections_request)
Returns all restaurant sections of specified terminal groups, for which banquet/reserve booking are available.

   > Allowed from version `7.1.5`.   > Restriction group: `Orders: preparing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_restaurant_sections_request** | Option<[**GetRestaurantSectionsRequest**](GetRestaurantSectionsRequest.md)> |  |  |

### Return type

[**models::GetRestaurantSectionsResponse**](GetRestaurantSectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_available_terminal_groups_post

> models::TerminalGroupsResponse api1_reserve_available_terminal_groups_post(authorization, timeout, get_terminal_groups_by_organizations_request)
Returns all terminal groups of specified organizations, for which banquet/reserve booking are available.

   > Allowed from version `7.1.5`.   > Restriction group: `Orders: preparing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_terminal_groups_by_organizations_request** | Option<[**GetTerminalGroupsByOrganizationsRequest**](GetTerminalGroupsByOrganizationsRequest.md)> |  |  |

### Return type

[**models::TerminalGroupsResponse**](TerminalGroupsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_cancel_post

> models::CorrelationIdResponse api1_reserve_cancel_post(authorization, timeout, cancel_reserve_request)
Cancel reservation due to some reason.

Available only for reserves with status 'New'.   > Allowed from version `8.2.6`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Order status: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**cancel_reserve_request** | Option<[**CancelReserveRequest**](CancelReserveRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_create_post

> models::ReserveResponse api1_reserve_create_post(authorization, timeout, create_reserve_request)
Create banquet/reserve.

   > Allowed from version `7.1.5`.   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Orders: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**create_reserve_request** | Option<[**CreateReserveRequest**](CreateReserveRequest.md)> |  |  |

### Return type

[**models::ReserveResponse**](ReserveResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_restaurant_sections_workload_post

> models::GetRestaurantSectionsWorkloadResponse api1_reserve_restaurant_sections_workload_post(authorization, timeout, get_restaurant_sections_workload_request)
Returns all banquets/reserves for passed restaurant sections.

   > Allowed from version `7.1.5`.   > Restriction group: `Data: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_restaurant_sections_workload_request** | Option<[**GetRestaurantSectionsWorkloadRequest**](GetRestaurantSectionsWorkloadRequest.md)> |  |  |

### Return type

[**models::GetRestaurantSectionsWorkloadResponse**](GetRestaurantSectionsWorkloadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_reserve_status_by_id_post

> models::ReservesResponse api1_reserve_status_by_id_post(authorization, timeout, reserves_by_id_request)
Retrieve banquets/reserves statuses by IDs.

   > Allowed from version `7.1.5`.   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**reserves_by_id_request** | Option<[**ReservesByIdRequest**](ReservesByIdRequest.md)> |  |  |

### Return type

[**models::ReservesResponse**](ReservesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

