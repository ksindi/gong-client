# \PermissionsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_users_access_to_calls**](PermissionsApi.md#add_users_access_to_calls) | **PUT** /v2/calls/users-access | Give individual users access to calls (/v2/calls/users-access)
[**create_permission_profile**](PermissionsApi.md#create_permission_profile) | **POST** /v2/permission-profile | Create permission profile (/v2/permission-profile)
[**delete_users_access_to_calls**](PermissionsApi.md#delete_users_access_to_calls) | **DELETE** /v2/calls/users-access | Remove specific individual users access from calls (/v2/calls/users-access)
[**get_permission_profile**](PermissionsApi.md#get_permission_profile) | **GET** /v2/permission-profile | Permission profile for a given profile Id (/v2/permission-profile)
[**get_users_access_to_calls**](PermissionsApi.md#get_users_access_to_calls) | **POST** /v2/calls/users-access | Retrieve users that have specific individual access to calls (/v2/calls/users-access)
[**list_permission_profile**](PermissionsApi.md#list_permission_profile) | **GET** /v2/all-permission-profiles | List all company permission profiles for a given workspace (/v2/all-permission-profiles)
[**list_permission_profile_users**](PermissionsApi.md#list_permission_profile_users) | **GET** /v2/permission-profile/users | List all users attached to a given permission profile (/v2/permission-profile/users)
[**update_permission_profile**](PermissionsApi.md#update_permission_profile) | **PUT** /v2/permission-profile | Update permission profile (/v2/permission-profile)



## add_users_access_to_calls

> crate::models::BaseResponse add_users_access_to_calls(calls_users_access_add_dto)
Give individual users access to calls (/v2/calls/users-access)

Give individual users access to calls.  If a user already has access (perhaps the call was shared with them, or they have access through their permission profiles) the request will have no effect.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:call-user-access:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calls_users_access_add_dto** | [**CallsUsersAccessAddDto**](CallsUsersAccessAddDto.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_permission_profile

> crate::models::PermissionProfileResponse create_permission_profile(workspace_id, permission_profile_dto)
Create permission profile (/v2/permission-profile)

Create a permission profile in a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace identifier.  You can retrieve the workspace using the \"workspaces\" (under \"Settings\") API. | [required] |
**permission_profile_dto** | [**PermissionProfileDto**](PermissionProfileDto.md) |  | [required] |

### Return type

[**crate::models::PermissionProfileResponse**](PermissionProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_users_access_to_calls

> crate::models::BaseResponse delete_users_access_to_calls(calls_users_access_delete_dto)
Remove specific individual users access from calls (/v2/calls/users-access)

Remove individual user access from calls. The request can only remove access previously given by the /v2/calls/users-access API.  If a given user does not have access to the call, they will be unaffected.  If a given user does have access to the call, but not through the pubic API (for example if the call was shared with the user), the user's access will remain unchanged.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:call-user-access:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**calls_users_access_delete_dto** | [**CallsUsersAccessDeleteDto**](CallsUsersAccessDeleteDto.md) |  | [required] |

### Return type

[**crate::models::BaseResponse**](BaseResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permission_profile

> crate::models::PermissionProfileResponse get_permission_profile(profile_id)
Permission profile for a given profile Id (/v2/permission-profile)

Returns a permission profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Permission profile identifier. | [required] |

### Return type

[**crate::models::PermissionProfileResponse**](PermissionProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_access_to_calls

> crate::models::CallsAccessDetailsResponse get_users_access_to_calls(filter_request_calls_access_get_dto)
Retrieve users that have specific individual access to calls (/v2/calls/users-access)

Returns a list of users who have received individual access to calls through the API.  This endpoint doesn't cover user that have access for other reasons (such as sharing recipients, or access through permission profiles).  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:call-user-access:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filter_request_calls_access_get_dto** | [**FilterRequestCallsAccessGetDto**](FilterRequestCallsAccessGetDto.md) |  | [required] |

### Return type

[**crate::models::CallsAccessDetailsResponse**](CallsAccessDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_permission_profile

> crate::models::PermissionProfilesResponse list_permission_profile(workspace_id)
List all company permission profiles for a given workspace (/v2/all-permission-profiles)

Returns a list of all permission profiles.  The listing is in the alphabetical order of the profile names.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** | Workspace identifier, the API will return only the profiles belonging to this workspace.  You can retrieve the workspace using the \"workspaces\" (under \"Settings\") API. | [required] |

### Return type

[**crate::models::PermissionProfilesResponse**](PermissionProfilesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_permission_profile_users

> crate::models::PermissionProfileUsersResponse list_permission_profile_users(profile_id)
List all users attached to a given permission profile (/v2/permission-profile/users)

Returns a list of all users whose access is controlled by the given permission profile.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:users:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Permission profile identifier. | [required] |

### Return type

[**crate::models::PermissionProfileUsersResponse**](PermissionProfileUsersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permission_profile

> crate::models::PermissionProfileResponse update_permission_profile(profile_id, permission_profile_dto)
Update permission profile (/v2/permission-profile)

Update a permission profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | Permission profile identifier. | [required] |
**permission_profile_dto** | [**PermissionProfileDto**](PermissionProfileDto.md) |  | [required] |

### Return type

[**crate::models::PermissionProfileResponse**](PermissionProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

