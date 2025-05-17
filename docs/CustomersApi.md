# \CustomersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_loyalty_iiko_customer_card_add_post**](CustomersApi.md#api1_loyalty_iiko_customer_card_add_post) | **POST** /api/1/loyalty/iiko/customer/card/add | Add card.
[**api1_loyalty_iiko_customer_card_remove_post**](CustomersApi.md#api1_loyalty_iiko_customer_card_remove_post) | **POST** /api/1/loyalty/iiko/customer/card/remove | Delete card.
[**api1_loyalty_iiko_customer_create_or_update_post**](CustomersApi.md#api1_loyalty_iiko_customer_create_or_update_post) | **POST** /api/1/loyalty/iiko/customer/create_or_update | Create or update customer.
[**api1_loyalty_iiko_customer_info_post**](CustomersApi.md#api1_loyalty_iiko_customer_info_post) | **POST** /api/1/loyalty/iiko/customer/info | Get customer info.
[**api1_loyalty_iiko_customer_program_add_post**](CustomersApi.md#api1_loyalty_iiko_customer_program_add_post) | **POST** /api/1/loyalty/iiko/customer/program/add | Add customer to program.
[**api1_loyalty_iiko_customer_wallet_cancel_hold_post**](CustomersApi.md#api1_loyalty_iiko_customer_wallet_cancel_hold_post) | **POST** /api/1/loyalty/iiko/customer/wallet/cancel_hold | Cancel hold money.
[**api1_loyalty_iiko_customer_wallet_chargeoff_post**](CustomersApi.md#api1_loyalty_iiko_customer_wallet_chargeoff_post) | **POST** /api/1/loyalty/iiko/customer/wallet/chargeoff | Withdraw balance.
[**api1_loyalty_iiko_customer_wallet_hold_post**](CustomersApi.md#api1_loyalty_iiko_customer_wallet_hold_post) | **POST** /api/1/loyalty/iiko/customer/wallet/hold | Hold money.
[**api1_loyalty_iiko_customer_wallet_topup_post**](CustomersApi.md#api1_loyalty_iiko_customer_wallet_topup_post) | **POST** /api/1/loyalty/iiko/customer/wallet/topup | Refill balance.
[**api1_loyalty_iiko_delete_customers_post**](CustomersApi.md#api1_loyalty_iiko_delete_customers_post) | **POST** /api/1/loyalty/iiko/delete_customers | Logical deletion of customers.
[**api1_loyalty_iiko_get_counters_post**](CustomersApi.md#api1_loyalty_iiko_get_counters_post) | **POST** /api/1/loyalty/iiko/get_counters | Get counters.
[**api1_loyalty_iiko_restore_customers_post**](CustomersApi.md#api1_loyalty_iiko_restore_customers_post) | **POST** /api/1/loyalty/iiko/restore_customers | Logical recovery of customers.



## api1_loyalty_iiko_customer_card_add_post

> serde_json::Value api1_loyalty_iiko_customer_card_add_post(authorization, timeout, add_magnet_card_request)
Add card.

Add new card for customer.   > Restriction group: `Guests: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_magnet_card_request** | Option<[**AddMagnetCardRequest**](AddMagnetCardRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_card_remove_post

> serde_json::Value api1_loyalty_iiko_customer_card_remove_post(authorization, timeout, delete_magnet_card_request)
Delete card.

Delete existing card for customer.   > Restriction group: `Guests: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**delete_magnet_card_request** | Option<[**DeleteMagnetCardRequest**](DeleteMagnetCardRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_create_or_update_post

> models::CreateOrUpdateCustomerResponse api1_loyalty_iiko_customer_create_or_update_post(authorization, timeout, create_or_update_customer_request)
Create or update customer.

Create or update customer info by id or phone or card track.   > Restriction group: `Guests: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**create_or_update_customer_request** | Option<[**CreateOrUpdateCustomerRequest**](CreateOrUpdateCustomerRequest.md)> |  |  |

### Return type

[**models::CreateOrUpdateCustomerResponse**](CreateOrUpdateCustomerResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_info_post

> models::GetCustomerInfoResponse api1_loyalty_iiko_customer_info_post(authorization, timeout, get_info_request)
Get customer info.

Get customer info by specified criterion.   > Restriction group: `Guests: info`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_info_request** | Option<[**GetInfoRequest**](GetInfoRequest.md)> |  |  |

### Return type

[**models::GetCustomerInfoResponse**](GetCustomerInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_program_add_post

> models::AddCustomerToProgramResponse api1_loyalty_iiko_customer_program_add_post(authorization, timeout, add_customer_to_program_request)
Add customer to program.

Add new customer for program.   > Restriction group: `Guests: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**add_customer_to_program_request** | Option<[**AddCustomerToProgramRequest**](AddCustomerToProgramRequest.md)> |  |  |

### Return type

[**models::AddCustomerToProgramResponse**](AddCustomerToProgramResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_wallet_cancel_hold_post

> serde_json::Value api1_loyalty_iiko_customer_wallet_cancel_hold_post(authorization, timeout, cancel_hold_money_request)
Cancel hold money.

Cancel holding transaction that created earlier.   > Restriction group: `Loyalty: wallets`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**cancel_hold_money_request** | Option<[**CancelHoldMoneyRequest**](CancelHoldMoneyRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_wallet_chargeoff_post

> serde_json::Value api1_loyalty_iiko_customer_wallet_chargeoff_post(authorization, timeout, change_user_balance_request)
Withdraw balance.

Withdraw customer balance.   > Restriction group: `Loyalty: wallets`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**change_user_balance_request** | Option<[**ChangeUserBalanceRequest**](ChangeUserBalanceRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_wallet_hold_post

> models::HoldMoneyResponse api1_loyalty_iiko_customer_wallet_hold_post(authorization, timeout, hold_money_request)
Hold money.

Hold customer's money in loyalty program. Payment will be process on POS during processing of an order.   > Restriction group: `Loyalty: wallets`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**hold_money_request** | Option<[**HoldMoneyRequest**](HoldMoneyRequest.md)> |  |  |

### Return type

[**models::HoldMoneyResponse**](HoldMoneyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_wallet_topup_post

> serde_json::Value api1_loyalty_iiko_customer_wallet_topup_post(authorization, timeout, change_user_balance_request)
Refill balance.

Refill customer balance.   > Restriction group: `Loyalty: wallets`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**change_user_balance_request** | Option<[**ChangeUserBalanceRequest**](ChangeUserBalanceRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_delete_customers_post

> models::DeleteCustomersResponse api1_loyalty_iiko_delete_customers_post(authorization, timeout, delete_customers_request)
Logical deletion of customers.

Mark customers as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**delete_customers_request** | Option<[**DeleteCustomersRequest**](DeleteCustomersRequest.md)> |  |  |

### Return type

[**models::DeleteCustomersResponse**](DeleteCustomersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_get_counters_post

> models::GetCountersResponse api1_loyalty_iiko_get_counters_post(authorization, timeout, get_counters_request)
Get counters.

Get customer orders count and sum for different .

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_counters_request** | Option<[**GetCountersRequest**](GetCountersRequest.md)> |  |  |

### Return type

[**models::GetCountersResponse**](GetCountersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_restore_customers_post

> models::RestoreCustomersResponse api1_loyalty_iiko_restore_customers_post(authorization, timeout, restore_customers_request)
Logical recovery of customers.

Removing deletion flags for customers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**restore_customers_request** | Option<[**RestoreCustomersRequest**](RestoreCustomersRequest.md)> |  |  |

### Return type

[**models::RestoreCustomersResponse**](RestoreCustomersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

