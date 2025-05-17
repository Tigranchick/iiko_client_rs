# \ReportApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_loyalty_iiko_customer_transactions_by_date_post**](ReportApi.md#api1_loyalty_iiko_customer_transactions_by_date_post) | **POST** /api/1/loyalty/iiko/customer/transactions/by_date | Get transaction report by .
[**api1_loyalty_iiko_customer_transactions_by_revision_post**](ReportApi.md#api1_loyalty_iiko_customer_transactions_by_revision_post) | **POST** /api/1/loyalty/iiko/customer/transactions/by_revision | Get transaction report by revision.



## api1_loyalty_iiko_customer_transactions_by_date_post

> models::GetTransactionsReportByResponse api1_loyalty_iiko_customer_transactions_by_date_post(authorization, timeout, get_transactions_report_by_request)
Get transaction report by .

Get transaction report for specified customer by provided date range.   > Restriction group: `Guests: info`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_transactions_report_by_request** | Option<[**GetTransactionsReportByRequest**](GetTransactionsReportByRequest.md)> |  |  |

### Return type

[**models::GetTransactionsReportByResponse**](GetTransactionsReportByResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_customer_transactions_by_revision_post

> models::GetTransactionsReportByRevisionResponse api1_loyalty_iiko_customer_transactions_by_revision_post(authorization, timeout, get_transactions_report_by_revision_request)
Get transaction report by revision.

Get transaction report for specified customer by provided revision.   > Restriction group: `Guests: info`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_transactions_report_by_revision_request** | Option<[**GetTransactionsReportByRevisionRequest**](GetTransactionsReportByRevisionRequest.md)> |  |  |

### Return type

[**models::GetTransactionsReportByRevisionResponse**](GetTransactionsReportByRevisionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

