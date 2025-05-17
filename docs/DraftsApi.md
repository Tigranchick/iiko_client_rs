# \DraftsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_deliveries_drafts_by_filter_post**](DraftsApi.md#api1_deliveries_drafts_by_filter_post) | **POST** /api/1/deliveries/drafts/by_filter | Retrieve order drafts list by parameters.
[**api1_deliveries_drafts_by_id_post**](DraftsApi.md#api1_deliveries_drafts_by_id_post) | **POST** /api/1/deliveries/drafts/by_id | Retrieve order draft by ID.
[**api1_deliveries_drafts_commit_post**](DraftsApi.md#api1_deliveries_drafts_commit_post) | **POST** /api/1/deliveries/drafts/commit | Admit order draft changes and send them to Front.
[**api1_deliveries_drafts_create_post**](DraftsApi.md#api1_deliveries_drafts_create_post) | **POST** /api/1/deliveries/drafts/create | Create delivery order draft.
[**api1_deliveries_drafts_delete_post**](DraftsApi.md#api1_deliveries_drafts_delete_post) | **POST** /api/1/deliveries/drafts/delete | Delete order draft.
[**api1_deliveries_drafts_lock_post**](DraftsApi.md#api1_deliveries_drafts_lock_post) | **POST** /api/1/deliveries/drafts/lock | Lock order draft.
[**api1_deliveries_drafts_save_post**](DraftsApi.md#api1_deliveries_drafts_save_post) | **POST** /api/1/deliveries/drafts/save | Update existing delivery order draft.
[**api1_deliveries_drafts_unlock_post**](DraftsApi.md#api1_deliveries_drafts_unlock_post) | **POST** /api/1/deliveries/drafts/unlock | Unlock order draft.



## api1_deliveries_drafts_by_filter_post

> models::RmsFilterDraftsResponse api1_deliveries_drafts_by_filter_post(authorization, timeout, rms_filter_drafts_request)
Retrieve order drafts list by parameters.

   > Restriction group: `Drafts: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_filter_drafts_request** | Option<[**RmsFilterDraftsRequest**](RmsFilterDraftsRequest.md)> |  |  |

### Return type

[**models::RmsFilterDraftsResponse**](RmsFilterDraftsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_by_id_post

> models::RmsGetDraftResponse api1_deliveries_drafts_by_id_post(authorization, timeout, rms_get_draft_request)
Retrieve order draft by ID.

   > Restriction group: `Drafts: receiving`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_get_draft_request** | Option<[**RmsGetDraftRequest**](RmsGetDraftRequest.md)> |  |  |

### Return type

[**models::RmsGetDraftResponse**](RmsGetDraftResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_commit_post

> models::OrderResponse api1_deliveries_drafts_commit_post(authorization, timeout, rms_commit_draft_request)
Admit order draft changes and send them to Front.

   > Restriction group: `Drafts: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_commit_draft_request** | Option<[**RmsCommitDraftRequest**](RmsCommitDraftRequest.md)> |  |  |

### Return type

[**models::OrderResponse**](OrderResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_create_post

> models::RmsCreateOrSaveDraftResponse api1_deliveries_drafts_create_post(authorization, timeout, rms_create_draft_request)
Create delivery order draft.

   > Restriction group: `Drafts: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_create_draft_request** | Option<[**RmsCreateDraftRequest**](RmsCreateDraftRequest.md)> |  |  |

### Return type

[**models::RmsCreateOrSaveDraftResponse**](RmsCreateOrSaveDraftResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_delete_post

> models::CorrelationIdResponse api1_deliveries_drafts_delete_post(authorization, timeout, rms_delete_draft_request)
Delete order draft.

   > Restriction group: `Drafts: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_delete_draft_request** | Option<[**RmsDeleteDraftRequest**](RmsDeleteDraftRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_lock_post

> models::CorrelationIdResponse api1_deliveries_drafts_lock_post(authorization, timeout, rms_lock_or_unlock_draft_request)
Lock order draft.

   > Restriction group: `Drafts: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_lock_or_unlock_draft_request** | Option<[**RmsLockOrUnlockDraftRequest**](RmsLockOrUnlockDraftRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_save_post

> models::RmsCreateOrSaveDraftResponse api1_deliveries_drafts_save_post(authorization, timeout, rms_save_draft_request)
Update existing delivery order draft.

   > Restriction group: `Drafts: creating`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_save_draft_request** | Option<[**RmsSaveDraftRequest**](RmsSaveDraftRequest.md)> |  |  |

### Return type

[**models::RmsCreateOrSaveDraftResponse**](RmsCreateOrSaveDraftResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_deliveries_drafts_unlock_post

> models::CorrelationIdResponse api1_deliveries_drafts_unlock_post(authorization, timeout, rms_lock_or_unlock_draft_request)
Unlock order draft.

   > Restriction group: `Drafts: changing`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**rms_lock_or_unlock_draft_request** | Option<[**RmsLockOrUnlockDraftRequest**](RmsLockOrUnlockDraftRequest.md)> |  |  |

### Return type

[**models::CorrelationIdResponse**](CorrelationIdResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

