# \DeliveriesRetrieveApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_deliveries_by_delivery_date_and_phone_post**](DeliveriesRetrieveApi.md#api1_deliveries_by_delivery_date_and_phone_post) | **POST** /api/1/deliveries/by_delivery_date_and_phone | Retrieve list of orders by telephone number, dates and revision.
[**api1_deliveries_by_delivery_date_and_source_key_and_filter_post**](DeliveriesRetrieveApi.md#api1_deliveries_by_delivery_date_and_source_key_and_filter_post) | **POST** /api/1/deliveries/by_delivery_date_and_source_key_and_filter | Search orders by search text and additional filters (date, problem, statuses and other).
[**api1_deliveries_by_delivery_date_and_status_post**](DeliveriesRetrieveApi.md#api1_deliveries_by_delivery_date_and_status_post) | **POST** /api/1/deliveries/by_delivery_date_and_status | Retrieve list of orders by statuses and dates.
[**api1_deliveries_by_id_post**](DeliveriesRetrieveApi.md#api1_deliveries_by_id_post) | **POST** /api/1/deliveries/by_id | Retrieve orders by IDs.
[**api1_deliveries_by_revision_post**](DeliveriesRetrieveApi.md#api1_deliveries_by_revision_post) | **POST** /api/1/deliveries/by_revision | Retrieve list of orders changed from the time revision was passed.
[**api1_deliveries_history_by_delivery_date_and_phone_post**](DeliveriesRetrieveApi.md#api1_deliveries_history_by_delivery_date_and_phone_post) | **POST** /api/1/deliveries/history/by_delivery_date_and_phone | Retrieve list of history orders by telephone number, dates and revision.



## api1_deliveries_by_delivery_date_and_phone_post

> models::OrdersWithRevisionResponse api1_deliveries_by_delivery_date_and_phone_post(authorization, timeout, request_orders_by_delivery_date_and_phone_request)
Retrieve list of orders by telephone number, dates and revision.

   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_orders_by_delivery_date_and_phone_request** | Option<[**RequestOrdersByDeliveryDateAndPhoneRequest**](RequestOrdersByDeliveryDateAndPhoneRequest.md)> |  |  |

### Return type

[**models::OrdersWithRevisionResponse**](OrdersWithRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_by_delivery_date_and_source_key_and_filter_post

> models::OrdersWithRevisionResponse api1_deliveries_by_delivery_date_and_source_key_and_filter_post(authorization, timeout, orders_by_delivery_date_and_filter_request)
Search orders by search text and additional filters (date, problem, statuses and other).

   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**orders_by_delivery_date_and_filter_request** | Option<[**OrdersByDeliveryDateAndFilterRequest**](OrdersByDeliveryDateAndFilterRequest.md)> |  |  |

### Return type

[**models::OrdersWithRevisionResponse**](OrdersWithRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_by_delivery_date_and_status_post

> models::OrdersWithRevisionResponse api1_deliveries_by_delivery_date_and_status_post(authorization, timeout, request_orders_by_delivery_date_and_status_request)
Retrieve list of orders by statuses and dates.

   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_orders_by_delivery_date_and_status_request** | Option<[**RequestOrdersByDeliveryDateAndStatusRequest**](RequestOrdersByDeliveryDateAndStatusRequest.md)> |  |  |

### Return type

[**models::OrdersWithRevisionResponse**](OrdersWithRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_by_id_post

> models::OrdersResponse api1_deliveries_by_id_post(authorization, timeout, request_orders_by_id_request)
Retrieve orders by IDs.

   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_orders_by_id_request** | Option<[**RequestOrdersByIdRequest**](RequestOrdersByIdRequest.md)> |  |  |

### Return type

[**models::OrdersResponse**](OrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_by_revision_post

> models::OrdersWithRevisionResponse api1_deliveries_by_revision_post(authorization, timeout, request_orders_by_revision_request)
Retrieve list of orders changed from the time revision was passed.

   > Restriction group: `Orders: receiving by revision`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_orders_by_revision_request** | Option<[**RequestOrdersByRevisionRequest**](RequestOrdersByRevisionRequest.md)> |  |  |

### Return type

[**models::OrdersWithRevisionResponse**](OrdersWithRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_history_by_delivery_date_and_phone_post

> models::OrdersWithRevisionResponse api1_deliveries_history_by_delivery_date_and_phone_post(authorization, timeout, request_orders_history_by_delivery_date_and_phone_request)
Retrieve list of history orders by telephone number, dates and revision.

   > Restriction group: `Orders: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**request_orders_history_by_delivery_date_and_phone_request** | Option<[**RequestOrdersHistoryByDeliveryDateAndPhoneRequest**](RequestOrdersHistoryByDeliveryDateAndPhoneRequest.md)> |  |  |

### Return type

[**models::OrdersWithRevisionResponse**](OrdersWithRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

