# \UsersApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user**](UsersApi.md#get_user) | **GET** /v2/users/{id} | Retrieve user (/v2/users/{id})
[**get_user_history**](UsersApi.md#get_user_history) | **GET** /v2/users/{id}/settings-history | Retrieve user settings history (/v2/users/{id}/settings-history)
[**list_multiple_users**](UsersApi.md#list_multiple_users) | **POST** /v2/users/extensive | List users by filter (/v2/users/extensive)
[**list_users**](UsersApi.md#list_users) | **GET** /v2/users | List all users (/v2/users)



## get_user

> crate::models::User get_user(id)
Retrieve user (/v2/users/{id})

Retrieve a specific user.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:users:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Gong's unique numeric identifier for the user (up to 20 digits). | [required] |

### Return type

[**crate::models::User**](User.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_history

> crate::models::SettingsHistory get_user_history(id)
Retrieve user settings history (/v2/users/{id}/settings-history)

Retrieve a specific user's settings history.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:users:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Gong's unique numeric identifier for the user (up to 20 digits). | [required] |

### Return type

[**crate::models::SettingsHistory**](SettingsHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_multiple_users

> crate::models::UsersMetadata list_multiple_users(public_api_base_request_v2_multiple_users_request_with_creation_dates)
List users by filter (/v2/users/extensive)

List multiple Users.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:users:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_multiple_users_request_with_creation_dates** | [**PublicApiBaseRequestV2MultipleUsersRequestWithCreationDates**](PublicApiBaseRequestV2MultipleUsersRequestWithCreationDates.md) |  | [required] |

### Return type

[**crate::models::UsersMetadata**](UsersMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> crate::models::UsersMetadata list_users(cursor, include_avatars)
List all users (/v2/users)

List all of the company's users.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:users:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |
**include_avatars** | Option<**bool**> | Avatars are synthetic users representing Gong employees (CSMs and support providers) when they access your instance. References to avatars' IDs may be found in the outputs of other API endpoints. This parameter is optional, if not provided avatars will not be included in the results. |  |

### Return type

[**crate::models::UsersMetadata**](UsersMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

