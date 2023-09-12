# \AuditingApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_logs**](AuditingApi.md#list_logs) | **GET** /v2/logs | Retrieve logs data by type and time range (/v2/logs)



## list_logs

> crate::models::LogsResponse list_logs(log_type, from_date_time, to_date_time, cursor)
Retrieve logs data by type and time range (/v2/logs)

List log entries that took place during a specified time range.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:logs:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**log_type** | **String** | Type of logs requested. | [required] |
**from_date_time** | **String** | The time from which to retrieve log records, in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC). | [required] |
**to_date_time** | Option<**String**> | The time until which to retrieve log records, in the ISO-8601 format (e.g., '2018-02-18T02:30:00-07:00' or '2018-02-18T08:00:00Z', where Z stands for UTC); if not specified, the logs end with the latest recorded log. |  |
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |

### Return type

[**crate::models::LogsResponse**](LogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

