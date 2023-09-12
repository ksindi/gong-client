# PermissionProfileDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Unique numeric identifier of the permission profile (up to 20 digits). | [optional]
**name** | Option<**String**> | Permission profile name. | [optional]
**description** | Option<**String**> | Permission profile description. | [optional]
**calls_access** | Option<[**crate::models::CallAccessWithPermissionLevel**](CallAccessWithPermissionLevel.md)> |  | [optional]
**library_folder_access** | Option<[**crate::models::LibraryFolderAccess**](LibraryFolderAccess.md)> |  | [optional]
**deals_access** | Option<[**crate::models::DealsAccessWithPermissionLevel**](DealsAccessWithPermissionLevel.md)> |  | [optional]
**forecast_permissions** | Option<[**crate::models::ForecastPermissions**](ForecastPermissions.md)> |  | [optional]
**coaching_access** | Option<[**crate::models::CoachingAccessWithPermissionLevel**](CoachingAccessWithPermissionLevel.md)> |  | [optional]
**insights_access** | Option<[**crate::models::InsightsAccessWithPermissionLevel**](InsightsAccessWithPermissionLevel.md)> |  | [optional]
**emails_access** | Option<[**crate::models::EmailsAccessWithPermissionLevel**](EmailsAccessWithPermissionLevel.md)> |  | [optional]
**score_calls** | Option<**bool**> | User can score calls. | [optional]
**download_call_media** | Option<**bool**> | User can download call media. | [optional]
**share_calls_with_customers** | Option<**bool**> | User can share calls with customers. | [optional]
**manually_schedule_and_upload_calls** | Option<**bool**> | User can manually schedule and upload calls. | [optional]
**private_calls** | Option<**bool**> | User can set their own calls as private. | [optional]
**delete_calls** | Option<**bool**> | User can delete calls. | [optional]
**trim_calls** | Option<**bool**> | User can trim calls. | [optional]
**delete_emails** | Option<**bool**> | User can delete emails. | [optional]
**calls_and_search** | Option<**bool**> | User can view and search calls. | [optional]
**library** | Option<**bool**> | User can view library pages. | [optional]
**deals** | Option<**bool**> | User can view deals pages. | [optional]
**create_edit_and_delete_deals_boards** | Option<**bool**> | User can create/edit/delete deals boards. | [optional]
**deals_inline_editing** | Option<**bool**> | User can perform inline editing of deals. | [optional]
**account** | Option<**bool**> | User can view account pages. | [optional]
**coaching** | Option<**bool**> | User can view coaching pages. | [optional]
**team_stats** | Option<**bool**> | User can view team stats page. | [optional]
**initiatives** | Option<**bool**> | User can view initiatives page. | [optional]
**market** | Option<**bool**> | User can view market page. | [optional]
**activity** | Option<**bool**> | User can view activity pages. | [optional]
**forecast** | Option<**bool**> | User can view forecast pages. | [optional]
**forecast_manage** | Option<**bool**> | User can manage forecast boards and upload targets. | [optional]
**engage_manage_company_templates** | Option<**bool**> | User can manage company email templates. | [optional]
**engage_manage_company_sequences** | Option<**bool**> | User can manage company sequences. | [optional]
**manage_general_business_settings** | Option<**bool**> | User can manage general business settings. | [optional]
**manage_scorecards** | Option<**bool**> | User can manage scorecards. | [optional]
**export_calls_and_coaching_data_to_csv** | Option<**bool**> | User can export calls and coaching metrics data to CSV. | [optional]
**crm_data_inline_editing** | Option<**bool**> | User can perform inline editing of crm data. | [optional]
**crm_data_import** | Option<**bool**> | User can perform import of crm data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


