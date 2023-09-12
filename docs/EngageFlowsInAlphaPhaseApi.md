# \EngageFlowsInAlphaPhaseApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_prospects**](EngageFlowsInAlphaPhaseApi.md#assign_prospects) | **POST** /v2/flows/prospects/assign | Assign a Gong Engage flow to prospects  (/v2/flows/prospects/assign)
[**get_flows_for_prospects**](EngageFlowsInAlphaPhaseApi.md#get_flows_for_prospects) | **POST** /v2/flows/prospects | List assigned flows for the given prospects (/v2/flows/prospects)
[**list_flows**](EngageFlowsInAlphaPhaseApi.md#list_flows) | **GET** /v2/flows | List Gong Engage flows (/v2/flows)



## assign_prospects

> crate::models::AssignFlowResponse assign_prospects(assign_flow_request_v2)
Assign a Gong Engage flow to prospects  (/v2/flows/prospects/assign)

Assign a Gong Engage flow to a batch of prospects.  Gong Engage requires 'prospectsCrmIds' to fetch the prospects from the CRM, the chosen 'gongFlowId' to assign to the prospect, and an 'assignToEmail' which is the email address of the Gong user for whom the flow tasks will be created.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:flows:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assign_flow_request_v2** | [**AssignFlowRequestV2**](AssignFlowRequestV2.md) |  | [required] |

### Return type

[**crate::models::AssignFlowResponse**](AssignFlowResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_flows_for_prospects

> crate::models::ProspectsFlowsResponse get_flows_for_prospects(prospects_assigned_flows_request_v2)
List assigned flows for the given prospects (/v2/flows/prospects)

Get the Gong Engage flows assigned to the given prospects.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:flows:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prospects_assigned_flows_request_v2** | [**ProspectsAssignedFlowsRequestV2**](ProspectsAssignedFlowsRequestV2.md) |  | [required] |

### Return type

[**crate::models::ProspectsFlowsResponse**](ProspectsFlowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_flows

> crate::models::FlowsResponse list_flows(flow_owner_email, cursor, workspace_id)
List Gong Engage flows (/v2/flows)

Lists all 'COMPANY' flows and optionally 'Personal' flows available for the user to choose from.   Gong Engage sorts flows by 2 visibility types: 'Personal' and 'Company'   When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:flows:read'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flow_owner_email** | **String** | Email of a Gong user, the API will return 'PERSONAL' flows belonging to the given user in addition to the 'COMPANY' flows. | [required] |
**cursor** | Option<**String**> | When paging is needed, provide the value supplied by the previous API call to bring the following page of records. |  |
**workspace_id** | Option<**String**> | Optional Workspace identifier, if supplied the API will return only the flows belonging to this workspace. |  |

### Return type

[**crate::models::FlowsResponse**](FlowsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

