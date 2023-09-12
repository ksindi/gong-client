# LibraryFolderAccess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**permission_level** | Option<**String**> | Library folder permission level - mandatory value. | [optional]
**library_folder_ids** | Option<**Vec<String>**> | List of library folder Ids, if set, \"permissionLevel\" must not be \"none\". | [optional]
**manage_public_folder** | Option<**bool**> | A user with this profile can manage public folder, if set, \"permissionLevel\" must not be \"none\". | [optional]
**manage_streams** | Option<**bool**> | A user with this profile can manage streams, if set, \"permissionLevel\" must not be \"none\". | [optional]
**manage_folder_calls** | Option<**bool**> | A user with this profile can add calls to folders, archive calls, if set, \"permissionLevel\" must not be \"none\". | [optional]
**share_folders_and_streams** | Option<**bool**> | A user with this profile can share folders and streams from Your Library with other team members, if set, \"permissionLevel\" must not be \"none\". | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


