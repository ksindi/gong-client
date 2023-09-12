# \LibraryApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_calls_in_specific_folder**](LibraryApi.md#get_calls_in_specific_folder) | **GET** /v2/library/folder-content | List of calls in a specific folder (/v2/library/folder-content)
[**get_library_structure**](LibraryApi.md#get_library_structure) | **GET** /v2/library/folders | Retrieve Library folders (/v2/library/folders)



## get_calls_in_specific_folder

> crate::models::LibraryFolderListOfCallsResponse get_calls_in_specific_folder(folder_id)
List of calls in a specific folder (/v2/library/folder-content)

Given a folder id, this endpoint retrieves a list of calls in it.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:library:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | Option<**String**> | Gong's unique numeric identifier for the folder (up to 20 digits). |  |

### Return type

[**crate::models::LibraryFolderListOfCallsResponse**](LibraryFolderListOfCallsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_library_structure

> crate::models::LibraryResponse get_library_structure(workspace_id)
Retrieve Library folders (/v2/library/folders)

Use this endpoint to retrieve a list of public library folders. We do not allow retrieval of either private or archived folders.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:library:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> | Workspace identifier. We will retrieve folders which are related to this specific workspace. |  |

### Return type

[**crate::models::LibraryResponse**](LibraryResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

