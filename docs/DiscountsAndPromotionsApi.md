# \DiscountsAndPromotionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_loyalty_iiko_calculate_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_calculate_post) | **POST** /api/1/loyalty/iiko/calculate | Calculate checkin.
[**api1_loyalty_iiko_coupons_by_series_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_coupons_by_series_post) | **POST** /api/1/loyalty/iiko/coupons/by_series | Get non-activated coupons
[**api1_loyalty_iiko_coupons_info_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_coupons_info_post) | **POST** /api/1/loyalty/iiko/coupons/info | Get coupon info.
[**api1_loyalty_iiko_coupons_series_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_coupons_series_post) | **POST** /api/1/loyalty/iiko/coupons/series | Get coupon series with non-activated coupons.
[**api1_loyalty_iiko_manual_condition_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_manual_condition_post) | **POST** /api/1/loyalty/iiko/manual_condition | Get manual conditions.
[**api1_loyalty_iiko_program_post**](DiscountsAndPromotionsApi.md#api1_loyalty_iiko_program_post) | **POST** /api/1/loyalty/iiko/program | Get programs.



## api1_loyalty_iiko_calculate_post

> models::CalculateCheckinResponse api1_loyalty_iiko_calculate_post(authorization, timeout, calculate_checkin_request)
Calculate checkin.

Calculate discounts and other loyalty items for an order.   > Restriction group: `Loyalty: order calculate`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**calculate_checkin_request** | Option<[**CalculateCheckinRequest**](CalculateCheckinRequest.md)> |  |  |

### Return type

[**models::CalculateCheckinResponse**](CalculateCheckinResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_coupons_by_series_post

> models::NotActivatedCouponResponse api1_loyalty_iiko_coupons_by_series_post(authorization, timeout, not_activated_coupon_request)
Get non-activated coupons

Get list of non-activated coupons.   > Restriction group: `Loyalty: coupons`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**not_activated_coupon_request** | Option<[**NotActivatedCouponRequest**](NotActivatedCouponRequest.md)> |  |  |

### Return type

[**models::NotActivatedCouponResponse**](NotActivatedCouponResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_coupons_info_post

> models::CouponInfoResponse api1_loyalty_iiko_coupons_info_post(authorization, timeout, coupon_info_request)
Get coupon info.

Get information about the specified coupon.   > Restriction group: `Loyalty: coupons`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**coupon_info_request** | Option<[**CouponInfoRequest**](CouponInfoRequest.md)> |  |  |

### Return type

[**models::CouponInfoResponse**](CouponInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_coupons_series_post

> models::SeriesWithNotActivatedCouponsResponse api1_loyalty_iiko_coupons_series_post(authorization, timeout, series_with_not_activated_coupons_request)
Get coupon series with non-activated coupons.

Get a list of coupon series in which there are not deleted and not activated coupons.   > Restriction group: `Loyalty: coupons`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**series_with_not_activated_coupons_request** | Option<[**SeriesWithNotActivatedCouponsRequest**](SeriesWithNotActivatedCouponsRequest.md)> |  |  |

### Return type

[**models::SeriesWithNotActivatedCouponsResponse**](SeriesWithNotActivatedCouponsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_manual_condition_post

> models::GetManualConditionsResponse api1_loyalty_iiko_manual_condition_post(authorization, timeout, get_by_organization_id_request)
Get manual conditions.

Get all organization's manual conditions.   > Restriction group: `Loyalty: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_by_organization_id_request** | Option<[**GetByOrganizationIdRequest**](GetByOrganizationIdRequest.md)> |  |  |

### Return type

[**models::GetManualConditionsResponse**](GetManualConditionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_loyalty_iiko_program_post

> models::GetProgramsResponse api1_loyalty_iiko_program_post(authorization, timeout, get_programs_request)
Get programs.

Get all loyalty programs for organization.   > Restriction group: `Loyalty: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_programs_request** | Option<[**GetProgramsRequest**](GetProgramsRequest.md)> |  |  |

### Return type

[**models::GetProgramsResponse**](GetProgramsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

