# InteractionMeasuresDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**speakers** | Option<[**Vec<crate::models::UserSpeaker>**](UserSpeaker.md)> | A list of the talk duration per speaker. | [optional]
**interaction_stats** | Option<[**Vec<crate::models::InteractionStatsDto>**](InteractionStatsDto.md)> | A list of interaction statistics. Applicable values: 'Longest Monologue', 'Longest Customer Story', 'Interactivity', 'Patience'. | [optional]
**video** | Option<[**Vec<crate::models::VideoInteractionDto>**](VideoInteractionDto.md)> | A list of video statistics about what's presented and for how long. | [optional]
**questions** | Option<[**crate::models::Questions**](Questions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


