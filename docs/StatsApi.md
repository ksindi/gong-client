# \StatsApi

All URIs are relative to *https://127.0.0.1:8443*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_answered_scorecards**](StatsApi.md#list_answered_scorecards) | **POST** /v2/stats/activity/scorecards | Retrieve answered scorecards for applicable reviewed users or scorecards for a date range (/v2/stats/activity/scorecards)
[**list_interaction_stats**](StatsApi.md#list_interaction_stats) | **POST** /v2/stats/interaction | Retrieve interaction stats for applicable users by date (/v2/stats/interaction)
[**list_multiple_users_aggregate_activity**](StatsApi.md#list_multiple_users_aggregate_activity) | **POST** /v2/stats/activity/aggregate | Retrieve aggregated activity for defined users by date (/v2/stats/activity/aggregate)
[**list_multiple_users_aggregate_by_period**](StatsApi.md#list_multiple_users_aggregate_by_period) | **POST** /v2/stats/activity/aggregate-by-period | Retrieve aggregated activity for defined users by a date range with grouping in time periods (/v2/stats/activity/aggregate-by-period)
[**list_multiple_users_day_by_day_activity**](StatsApi.md#list_multiple_users_day_by_day_activity) | **POST** /v2/stats/activity/day-by-day | Retrieve daily activity for applicable users for a date range (/v2/stats/activity/day-by-day)



## list_answered_scorecards

> crate::models::AnsweredScorecards list_answered_scorecards(public_api_base_request_v2_answered_scorecards_filter)
Retrieve answered scorecards for applicable reviewed users or scorecards for a date range (/v2/stats/activity/scorecards)

Retrieve all the answers for the scorecards that were reviewed during a specified date range, for calls that took place during a specified date range, for specific scorecards or for specific reviewed users.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:stats:scorecards'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_answered_scorecards_filter** | [**PublicApiBaseRequestV2AnsweredScorecardsFilter**](PublicApiBaseRequestV2AnsweredScorecardsFilter.md) |  | [required] |

### Return type

[**crate::models::AnsweredScorecards**](AnsweredScorecards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_interaction_stats

> crate::models::CompanyUsersIntercationStatsResponse list_interaction_stats(public_api_base_request_v2_multiple_users_with_dates)
Retrieve interaction stats for applicable users by date (/v2/stats/interaction)

Returns interaction stats for users based on calls that have Whisper turned on.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:stats:interaction'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_multiple_users_with_dates** | [**PublicApiBaseRequestV2MultipleUsersWithDates**](PublicApiBaseRequestV2MultipleUsersWithDates.md) |  | [required] |

### Return type

[**crate::models::CompanyUsersIntercationStatsResponse**](CompanyUsersIntercationStatsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_multiple_users_aggregate_activity

> crate::models::CompanyUsersAggregateActivityResponse list_multiple_users_aggregate_activity(public_api_base_request_v2_multiple_users_with_dates)
Retrieve aggregated activity for defined users by date (/v2/stats/activity/aggregate)

Lists the activity of multiple users within the Gong system during a defined period. Given the period, this endpoint returns multiple records, one for each user, with an applicable activity during the period. Each record includes statistics about the user's activity.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:stats:user-actions'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_multiple_users_with_dates** | [**PublicApiBaseRequestV2MultipleUsersWithDates**](PublicApiBaseRequestV2MultipleUsersWithDates.md) |  | [required] |

### Return type

[**crate::models::CompanyUsersAggregateActivityResponse**](CompanyUsersAggregateActivityResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_multiple_users_aggregate_by_period

> crate::models::UsersAggregateByPeriodActivity list_multiple_users_aggregate_by_period(request_with_time_period)
Retrieve aggregated activity for defined users by a date range with grouping in time periods (/v2/stats/activity/aggregate-by-period)

Lists the aggregated activity of multiple users within the Gong system for each time period within the defined date range. This endpoint returns multiple records, one for each user. For each user there are items for every time period in the date range, including statistics about the user's activity. Records are returned only for users with activity in the range.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:stats:user-actions'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_with_time_period** | [**RequestWithTimePeriod**](RequestWithTimePeriod.md) |  | [required] |

### Return type

[**crate::models::UsersAggregateByPeriodActivity**](UsersAggregateByPeriodActivity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_multiple_users_day_by_day_activity

> crate::models::UsersDayByDayActivity list_multiple_users_day_by_day_activity(public_api_base_request_v2_multiple_users_with_dates)
Retrieve daily activity for applicable users for a date range (/v2/stats/activity/day-by-day)

Retrieve the daily activity of multiple users within the Gong system for a range of dates. This endpoint returns records including statistics about each user's activity, on the daily level. Records are returned only for users with activity in the range.  When accessed through a Bearer token authorization method, this endpoint requires the scope 'api:stats:user-actions:detailed'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**public_api_base_request_v2_multiple_users_with_dates** | [**PublicApiBaseRequestV2MultipleUsersWithDates**](PublicApiBaseRequestV2MultipleUsersWithDates.md) |  | [required] |

### Return type

[**crate::models::UsersDayByDayActivity**](UsersDayByDayActivity.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

