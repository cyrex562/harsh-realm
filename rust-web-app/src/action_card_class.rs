// Decompiled with JetBrains decompiler
#![allow(non_snake_case)]
// Type: WindowsApplication1.ActionCardClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// using Microsoft.VisualBasic.CompilerServices,
// using System,
// using System.IO,
// using System.Runtime.Serialization,
// using System.Runtime.Serialization.Formatters.Binary,

use crate::lib_id_class::LibIdClass;

// namespace WindowsApplication1
// {
// [Serializable]
//pub class ActionCardClass : ISerializable
#[derive(Debug, Clone, Default)]
pub struct ActionCardClass {
    pub Title: String,
    pub ColorScheme: i32,
    pub Text: String,
    pub MouseOver: String,
    pub PPCost: i32,
    pub ExecuteEvent: i32,
    pub AILabel: i32,
    pub AILabel2: i32,
    pub aILabel3: i32,
    pub EventPicNr: i32,
    pub AlternateEventPicNr: i32,
    pub TempVar0: i32,
    pub TempVar1: i32,
    pub AreaSlot: i32,
    pub AreaValue: i32,
    pub PreExecuteEvent: i32,
    pub UnitSelect: bool,
    pub Nato: i32,
    pub SmallGfx: i32,
    pub HisVarCostType: i32,
    pub HisVarCostQty: i32,
    pub Category: i32,
    pub LimitedShow: i32,
    pub LibId: LibIdClass,
    pub IgnorePopupIfNoSelect: bool,
    pub quickButton: i32,
    pub quickSmall: i32,
    pub customCostType: i32,
    pub customCostQty: i32,
}

impl ActionCardClass {
    // SelfActionCardClass()
    pub fn new() -> Self {
        Self {
            Title: "Empty Historical".to_string(),
            MouseOver: "".to_string(),
            EventPicNr: -1,
            SmallGfx: -1,
            LibId: LibIdClass::new(),
            ColorScheme: 0,
            ExecuteEvent: -1,
            TempVar0: -1,
            TempVar1: -1,
            AILabel2: 0,
            AreaSlot: -1,
            AreaValue: -1,
            PreExecuteEvent: -1,
            UnitSelect: false,
            Nato: -1,
            HisVarCostQty: 0,
            HisVarCostType: -1,
            Category: -1,
            quickSmall: -1,
            quickButton: 0,
            customCostQty: 0,
            customCostType: 0,
            Text: "".to_string(),
            PPCost: 0,
            AILabel: 0,
            aILabel3: 0,
            AlternateEventPicNr: 0,
            LimitedShow: 0,
            IgnorePopupIfNoSelect: false,
        }
    }

    // pub ActionCardClass Clone()
    // {
    //   BinaryFormatter binaryFormatter = BinaryFormatter::new(),
    //   MemoryStream serializationStream = MemoryStream::new(),
    //   binaryFormatter.Serialize((Stream) serializationStream,  this),
    //   serializationStream.Position = 0L,
    //   return (ActionCardClass) binaryFormatter.Deserialize((Stream) serializationStream),
    // }

    // pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    // {
    //   info.AddValue("Title",  Title),
    //   info.AddValue("ColorScheme", ColorScheme),
    //   info.AddValue("Text",  Text),
    //   info.AddValue("PPCost", PPCost),
    //   info.AddValue("ExecuteEvent", ExecuteEvent),
    //   info.AddValue("AILabel", AILabel),
    //   info.AddValue("AILabel2", AILabel2),
    //   info.AddValue("AILabel3", aILabel3),
    //   info.AddValue("EventPicNr", EventPicNr),
    //   info.AddValue("TempVar0", TempVar0),
    //   info.AddValue("TempVar1", TempVar1),
    //   info.AddValue("AreaSlot", AreaSlot),
    //   info.AddValue("AreaValue", AreaValue),
    //   info.AddValue("PreExecuteEvent", PreExecuteEvent),
    //   info.AddValue("UnitSelect", UnitSelect),
    //   info.AddValue("Nato", Nato),
    //   info.AddValue("MouseOver",  MouseOver),
    //   info.AddValue("HisVarCostType", HisVarCostType),
    //   info.AddValue("HisVarCostQty", HisVarCostQty),
    //   info.AddValue("Category", Category),
    //   info.AddValue("AlternateEventPicNr", AlternateEventPicNr),
    //   info.AddValue("LimitedShow", LimitedShow),
    //   info.AddValue("ourid",  LibId),
    //   info.AddValue("SmallGfx", SmallGfx),
    //   info.AddValue("IgnorePopupIfNoSelect", IgnorePopupIfNoSelect),
    //   info.AddValue("QuickSmall", quickSmall),
    //   info.AddValue("QuickButton", quickButton),
    //   info.AddValue("customCostType", customCostType),
    //   info.AddValue("customCostQty", customCostQty),
    // }

    // protected ActionCardClass(SerializationInfo info, StreamingContext context)
    // {
    //   Title = info.GetString(nameof (Title)),
    //   ColorScheme = info.GetInt32(nameof (ColorScheme)),
    //   Text = info.GetString(nameof (Text)),
    //   PPCost = info.GetInt32(nameof (PPCost)),
    //   ExecuteEvent = info.GetInt32(nameof (ExecuteEvent)),
    //   AILabel = info.GetInt32(nameof (AILabel)),
    //   EventPicNr = info.GetInt32(nameof (EventPicNr)),
    //   try
    //   {
    //     TempVar0 = info.GetInt32(nameof (TempVar0)),
    //     TempVar1 = info.GetInt32(nameof (TempVar1)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     TempVar0 = -1,
    //     TempVar1 = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     AreaSlot = info.GetInt32(nameof (AreaSlot)),
    //     AreaValue = info.GetInt32(nameof (AreaValue)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     AreaSlot = -1,
    //     AreaValue = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     PreExecuteEvent = info.GetInt32(nameof (PreExecuteEvent)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     PreExecuteEvent = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     UnitSelect = info.GetBoolean(nameof (UnitSelect)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     UnitSelect = false,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     Nato = info.GetInt32(nameof (Nato)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     Nato = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     MouseOver = info.GetString(nameof (MouseOver)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     MouseOver = "",
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     HisVarCostType = info.GetInt32(nameof (HisVarCostType)),
    //     HisVarCostQty = info.GetInt32(nameof (HisVarCostQty)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     HisVarCostQty = 0,
    //     HisVarCostType = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     Category = info.GetInt32(nameof (Category)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     Category = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     AlternateEventPicNr = info.GetInt32(nameof (AlternateEventPicNr)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     AlternateEventPicNr = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     LimitedShow = info.GetInt32(nameof (LimitedShow)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     LimitedShow = 0,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     AILabel2 = info.GetInt32(nameof (AILabel2)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     AILabel2 = 0,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     aILabel3 = info.GetInt32("AILabel3"),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     aILabel3 = 0,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     LibId = LibIdClass::new(),
    //     LibId = (LibIdClass) info.GetValue("ourid", LibId.GetType()),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     LibId = LibIdClass::new(),
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     SmallGfx = info.GetInt32(nameof (SmallGfx)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     SmallGfx = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     IgnorePopupIfNoSelect = info.GetBoolean(nameof (IgnorePopupIfNoSelect)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     IgnorePopupIfNoSelect = false,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     quickButton = info.GetInt32("QuickButton"),
    //     quickSmall = info.GetInt32("QuickSmall"),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     quickButton = 0,
    //     quickSmall = -1,
    //     ProjectData.ClearProjectError(),
    //   }
    //   try
    //   {
    //     customCostType = info.GetInt32(nameof (customCostType)),
    //     customCostQty = info.GetInt32(nameof (customCostQty)),
    //   }
    //   catch (Exception ex)
    //   {
    //     ProjectData.SetProjectError(ex),
    //     customCostType = 0,
    //     customCostQty = 0,
    //     ProjectData.ClearProjectError(),
    //   }
    // }
}
// }
