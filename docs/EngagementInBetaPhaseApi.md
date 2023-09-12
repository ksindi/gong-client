# \EngagementInBetaPhaseApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_shared**](EngagementInBetaPhaseApi.md#content_shared) | **PUT** /v2/customer-engagement/content/shared | Report event of a content share (/v2/customer-engagement/content/shared)
[**content_viewed**](EngagementInBetaPhaseApi.md#content_viewed) | **PUT** /v2/customer-engagement/content/viewed | Report event of a content view (/v2/customer-engagement/content/viewed)
[**custom_action**](EngagementInBetaPhaseApi.md#custom_action) | **PUT** /v2/customer-engagement/action | Report event of a custom action (/v2/customer-engagement/action)



## content_shared

> crate::models::RegisterGenericCrmResponse content_shared(content_shared_event)
Report event of a content share (/v2/customer-engagement/content/shared)

Push engagement events into Gong and display them as events in Gong’s activity timeline, when a Gong user shares content with external participants (for example, a contract was “shared” by the account executive with his prospects)  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:engagement-data:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_shared_event** | [**ContentSharedEvent**](ContentSharedEvent.md) |  | [required] |

### Return type

[**crate::models::RegisterGenericCrmResponse**](RegisterGenericCrmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_viewed

> crate::models::RegisterGenericCrmResponse content_viewed(content_viewed_event)
Report event of a content view (/v2/customer-engagement/content/viewed)

Push engagement events into Gong and display them as events in Gong’s activity timeline, when a content is viewed by an external participant (for example, a contract was “viewed” by the prospect)  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:engagement-data:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_viewed_event** | [**ContentViewedEvent**](ContentViewedEvent.md) |  | [required] |

### Return type

[**crate::models::RegisterGenericCrmResponse**](RegisterGenericCrmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_action

> crate::models::RegisterGenericCrmResponse custom_action(custom_action_event)
Report event of a custom action (/v2/customer-engagement/action)

Push engagement events into Gong and display them as events in Gong’s activity timeline, when a content is engaged by an external participant (for example, a contract was “signed” by the prospect)  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:engagement-data:write'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**custom_action_event** | [**CustomActionEvent**](CustomActionEvent.md) |  | [required] |

### Return type

[**crate::models::RegisterGenericCrmResponse**](RegisterGenericCrmResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

