# \EmployeesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api1_employees_couriers_active_location_by_terminal_post**](EmployeesApi.md#api1_employees_couriers_active_location_by_terminal_post) | **POST** /api/1/employees/couriers/active_location/by_terminal | Returns list of all active (courier session is opened) courier's locations which are delivery drivers in specified   restaurant and are clocked in on specified delivery terminal.
[**api1_employees_couriers_active_location_post**](EmployeesApi.md#api1_employees_couriers_active_location_post) | **POST** /api/1/employees/couriers/active_location | Returns list of all active (courier session is opened) courier's locations which are delivery drivers   in specified restaurants.
[**api1_employees_couriers_by_role_post**](EmployeesApi.md#api1_employees_couriers_by_role_post) | **POST** /api/1/employees/couriers/by_role | Returns list of all employees which are delivery drivers in specified restaurants,   and checks whether each employee has passed role.
[**api1_employees_couriers_locations_by_time_offset_post**](EmployeesApi.md#api1_employees_couriers_locations_by_time_offset_post) | **POST** /api/1/employees/couriers/locations/by_time_offset | Method of obtaining drivers' coordinates history.
[**api1_employees_couriers_post**](EmployeesApi.md#api1_employees_couriers_post) | **POST** /api/1/employees/couriers | Returns list of all employees which are delivery drivers in specified restaurants.
[**api1_employees_info_post**](EmployeesApi.md#api1_employees_info_post) | **POST** /api/1/employees/info | Returns employee info.
[**api1_employees_shift_clockin_post**](EmployeesApi.md#api1_employees_shift_clockin_post) | **POST** /api/1/employees/shift/clockin | Open personal session.
[**api1_employees_shift_clockout_post**](EmployeesApi.md#api1_employees_shift_clockout_post) | **POST** /api/1/employees/shift/clockout | Close personal session.
[**api1_employees_shift_is_open_post**](EmployeesApi.md#api1_employees_shift_is_open_post) | **POST** /api/1/employees/shift/is_open | Check if personal session is open.
[**api1_employees_shifts_by_courier_post**](EmployeesApi.md#api1_employees_shifts_by_courier_post) | **POST** /api/1/employees/shifts/by_courier | Get terminal groups where employee session is opened.



## api1_employees_couriers_active_location_by_terminal_post

> models::ActiveCourierLocationsResponse api1_employees_couriers_active_location_by_terminal_post(authorization, timeout, active_courier_locations_by_terminal_group_request)
Returns list of all active (courier session is opened) courier's locations which are delivery drivers in specified   restaurant and are clocked in on specified delivery terminal.

   > Restriction group: `Drivers: location`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**active_courier_locations_by_terminal_group_request** | Option<[**ActiveCourierLocationsByTerminalGroupRequest**](ActiveCourierLocationsByTerminalGroupRequest.md)> |  |  |

### Return type

[**models::ActiveCourierLocationsResponse**](ActiveCourierLocationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_couriers_active_location_post

> models::ActiveCourierLocationsResponse api1_employees_couriers_active_location_post(authorization, timeout, couriers_request)
Returns list of all active (courier session is opened) courier's locations which are delivery drivers   in specified restaurants.

   > Restriction group: `Drivers: location`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**couriers_request** | Option<[**CouriersRequest**](CouriersRequest.md)> |  |  |

### Return type

[**models::ActiveCourierLocationsResponse**](ActiveCourierLocationsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_couriers_by_role_post

> models::EmployeesWithRoleSignResponse api1_employees_couriers_by_role_post(authorization, timeout, couriers_and_check_role_request)
Returns list of all employees which are delivery drivers in specified restaurants,   and checks whether each employee has passed role.

   > Restriction group: `Drivers: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**couriers_and_check_role_request** | Option<[**CouriersAndCheckRoleRequest**](CouriersAndCheckRoleRequest.md)> |  |  |

### Return type

[**models::EmployeesWithRoleSignResponse**](EmployeesWithRoleSignResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_couriers_locations_by_time_offset_post

> models::CourierLocationsByTimeOffsetResponse api1_employees_couriers_locations_by_time_offset_post(authorization, timeout, courier_locations_by_time_offset_request)
Method of obtaining drivers' coordinates history.

   > Restriction group: `Drivers: location`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**courier_locations_by_time_offset_request** | Option<[**CourierLocationsByTimeOffsetRequest**](CourierLocationsByTimeOffsetRequest.md)> |  |  |

### Return type

[**models::CourierLocationsByTimeOffsetResponse**](CourierLocationsByTimeOffsetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_couriers_post

> models::EmployeesResponse api1_employees_couriers_post(authorization, timeout, couriers_request)
Returns list of all employees which are delivery drivers in specified restaurants.

   > Restriction group: `Drivers: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**couriers_request** | Option<[**CouriersRequest**](CouriersRequest.md)> |  |  |

### Return type

[**models::EmployeesResponse**](EmployeesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_info_post

> models::EmployeeInfoResponse api1_employees_info_post(authorization, timeout, employee_info_request)
Returns employee info.

   > Restriction group: `Employees: dictionaries`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**employee_info_request** | Option<[**EmployeeInfoRequest**](EmployeeInfoRequest.md)> |  |  |

### Return type

[**models::EmployeeInfoResponse**](EmployeeInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_shift_clockin_post

> models::ChangePersonalSessionResponse api1_employees_shift_clockin_post(authorization, timeout, open_personal_session_request)
Open personal session.

   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Employees: shifts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**open_personal_session_request** | Option<[**OpenPersonalSessionRequest**](OpenPersonalSessionRequest.md)> |  |  |

### Return type

[**models::ChangePersonalSessionResponse**](ChangePersonalSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_shift_clockout_post

> models::ChangePersonalSessionResponse api1_employees_shift_clockout_post(authorization, timeout, close_personal_session_request)
Close personal session.

   > This method is a command. Use `api/1/commands/status` method to get the progress status.   > Restriction group: `Employees: shifts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**close_personal_session_request** | Option<[**ClosePersonalSessionRequest**](ClosePersonalSessionRequest.md)> |  |  |

### Return type

[**models::ChangePersonalSessionResponse**](ChangePersonalSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_shift_is_open_post

> models::GetPersonalSessionInfoResponse api1_employees_shift_is_open_post(authorization, timeout, get_personal_session_info_request)
Check if personal session is open.

   > Restriction group: `Employees: shifts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_personal_session_info_request** | Option<[**GetPersonalSessionInfoRequest**](GetPersonalSessionInfoRequest.md)> |  |  |

### Return type

[**models::GetPersonalSessionInfoResponse**](GetPersonalSessionInfoResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api1_employees_shifts_by_courier_post

> models::GetTerminalGroupsOfEmployeeResponse api1_employees_shifts_by_courier_post(authorization, timeout, get_terminal_groups_of_employee_request)
Get terminal groups where employee session is opened.

   > Restriction group: `Employees: shifts`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Authorization token. | [required] |
**timeout** | Option<**i32**> | Timeout in seconds. |  |[default to 15]
**get_terminal_groups_of_employee_request** | Option<[**GetTerminalGroupsOfEmployeeRequest**](GetTerminalGroupsOfEmployeeRequest.md)> |  |  |

### Return type

[**models::GetTerminalGroupsOfEmployeeResponse**](GetTerminalGroupsOfEmployeeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

