# \SettingsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_scorecards**](SettingsApi.md#list_scorecards) | **GET** /v2/settings/scorecards | Retrieve scorecards details (/v2/settings/scorecards)
[**list_trackers**](SettingsApi.md#list_trackers) | **GET** /v2/settings/trackers | Retrieve tracker details (/v2/settings/trackers)
[**list_workspaces**](SettingsApi.md#list_workspaces) | **GET** /v2/workspaces | List all company workspaces (/v2/workspaces)



## list_scorecards

> crate::models::Scorecards list_scorecards()
Retrieve scorecards details (/v2/settings/scorecards)

Retrieve all the scorecards within the Gong system.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:settings:scorecards:read'.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Scorecards**](Scorecards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_trackers

> crate::models::Trackers list_trackers(workspace_id)
Retrieve tracker details (/v2/settings/trackers)

Retrieves details of all keyword trackers in the system or in a given workspace.  When accessed through a Bearer token authorization method, this endpoint requires the  'api:settings:trackers:read' scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> | The ID of the workspace the keyword trackers are in. When empty, all trackers in all workspaces are returned. |  |

### Return type

[**crate::models::Trackers**](Trackers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workspaces

> crate::models::WorkspacesMetadata list_workspaces()
List all company workspaces (/v2/workspaces)

Returns a list of all workspaces including their details.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:workspaces:read'.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::WorkspacesMetadata**](WorkspacesMetadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

