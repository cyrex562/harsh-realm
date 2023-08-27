// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LocationTypeClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class LocationTypeClass : ISerializable
  {
    pub Name: String;
    pub OverdrawLTNr: i32;
    pub OverdrawSpriteNr: i32;
    pub ExtraGraphic: i32;
    pub SmallGraphic: i32;
    pub SmallGraphicSpecialMode: i32;
    pub SmallGraphicSpecialData: i32;
    pub BuildgroundType: Vec<bool>;
    pub PeopleGroup: Vec<bool>;
    pub ItemGroup: Vec<bool>;
    pub MaxProd: i32;
    pub IsPort: bool;
    pub IsAirfield: bool;
    pub StructuralPts: i32;
    pub OnDestructLT: i32;
    pub OnDestructSpriteNr: i32;
    pub AutoRecoverPts: i32;
    pub Invincible: bool;
    pub UpgradeNr: i32;
    pub DowngradeNr: i32;
    pub LocTypeGroup: i32;
    pub MinDistance: Vec<i32>;
    pub Buildable: bool;
    pub SupplyCost: i32;
    pub PPCost: i32;
    pub EPCost: i32;
    pub ZOrder: i32;
    pub Research: Vec<i32>;
    pub VarType: Vec<i32>;
    pub VarQty: Vec<i32>;
    pub Description: String;
    pub SlotType: i32;
    pub SlotValue: i32;
    pub PictureLT: i32;
    pub PictureSprite: i32;
    pub AIPriority: i32;
    pub AIEvent: i32;
    pub AILocEvent: i32;
    pub AIAfterBuildEvent: i32;
    pub AICanBuild: bool;
    pub HumanCanBuild: bool;
    pub SetPeopleToSlotX: i32;
    pub NoHQ: bool;
    pub AutoProd: i32;
    pub TopAirStack: i32;
    pub Repairable: bool;
    pub Logistical: i32;
    pub useSmallLabel: bool;
    pub isSupplyBase: bool;
    pub isSupplySource: bool;
    pub maxSupply: i32;
    pub maxFuel: i32;
    pub maxEvacuate: i32;
    pub maxDestroy: i32;
    pub needsCityLevel: i32;
    pub cityLevel: i32;
    pub supplyRange: i32;
    pub editorBlock: bool;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("OverdrawLTNr", this.OverdrawLTNr);
      info.AddValue("OverdrawSpriteNr", this.OverdrawSpriteNr);
      info.AddValue("BuildgroundType",  this.BuildgroundType);
      info.AddValue("PeopleGroup",  this.PeopleGroup);
      info.AddValue("ItemGroup",  this.ItemGroup);
      info.AddValue("Repairable", this.Repairable);
      info.AddValue("MaxProd", this.MaxProd);
      info.AddValue("IsPort", this.IsPort);
      info.AddValue("IsAirfield", this.IsAirfield);
      info.AddValue("StructuralPts", this.StructuralPts);
      info.AddValue("OnDestructLT", this.OnDestructLT);
      info.AddValue("OnDestructSpriteNr", this.OnDestructSpriteNr);
      info.AddValue("AutoRecoverPts", this.AutoRecoverPts);
      info.AddValue("LocTypeGroup", this.LocTypeGroup);
      info.AddValue("MinDistance",  this.MinDistance);
      info.AddValue("Buildable", this.Buildable);
      info.AddValue("EPCost", this.EPCost);
      info.AddValue("Invincible", this.Invincible);
      info.AddValue("UpgradeNr", this.UpgradeNr);
      info.AddValue("DowngradeNr", this.DowngradeNr);
      info.AddValue("SupplyCost", this.SupplyCost);
      info.AddValue("PPCost", this.PPCost);
      info.AddValue("ZOrder", this.ZOrder);
      info.AddValue("ExtraGraphic", this.ExtraGraphic);
      info.AddValue("SmallGraphic", this.SmallGraphic);
      info.AddValue("SmallGraphicSpecialMode", this.SmallGraphicSpecialMode);
      info.AddValue("SmallGraphicSpecialData", this.SmallGraphicSpecialData);
      info.AddValue("Research",  this.Research);
      info.AddValue("VarType",  this.VarType);
      info.AddValue("VarQty",  this.VarQty);
      info.AddValue("Description",  this.Description);
      info.AddValue("SlotType", this.SlotType);
      info.AddValue("SlotValue", this.SlotValue);
      info.AddValue("PictureLT", this.PictureLT);
      info.AddValue("PictureSprite", this.PictureSprite);
      info.AddValue("AIPriority", this.AIPriority);
      info.AddValue("AIEvent", this.AIEvent);
      info.AddValue("AILocEvent", this.AILocEvent);
      info.AddValue("AICanBuild", this.AICanBuild);
      info.AddValue("HumanCanBuild", this.HumanCanBuild);
      info.AddValue("SetPeopleToSlotX", this.SetPeopleToSlotX);
      info.AddValue("NoHQ", this.NoHQ);
      info.AddValue("AutoProd", this.AutoProd);
      info.AddValue("AIAfterBuildEvent", this.AIAfterBuildEvent);
      info.AddValue("TopAirStack", this.TopAirStack);
      info.AddValue("Logistical", this.Logistical);
      info.AddValue("UseSmallLabel", this.useSmallLabel);
      info.AddValue("isSupplyBase", this.isSupplyBase);
      info.AddValue("maxSupply", this.maxSupply);
      info.AddValue("maxFuel", this.maxFuel);
      info.AddValue("maxEvacuate", this.maxEvacuate);
      info.AddValue("maxDestroy", this.maxDestroy);
      info.AddValue("needsCityLevel", this.needsCityLevel);
      info.AddValue("cityLevel", this.cityLevel);
      info.AddValue("supplyRange", this.supplyRange);
      info.AddValue("isSupplySource", this.isSupplySource);
      info.AddValue("editorBlock", this.editorBlock);
    }

    protected LocationTypeClass(SerializationInfo info, StreamingContext context)
    {
      this.BuildgroundType = new bool[100];
      this.PeopleGroup = new bool[100];
      this.ItemGroup = new bool[100];
      this.MinDistance = new int[100];
      this.Research = new int[5];
      this.VarType = new int[5];
      this.VarQty = new int[5];
      this.Name = info.GetString(nameof (Name));
      this.OverdrawLTNr = info.GetInt32(nameof (OverdrawLTNr));
      this.OverdrawSpriteNr = info.GetInt32(nameof (OverdrawSpriteNr));
      this.BuildgroundType = (bool[]) info.GetValue(nameof (BuildgroundType), this.BuildgroundType.GetType());
      this.BuildgroundType = (bool[]) Utils.CopyArray((Array) this.BuildgroundType, (Array) new bool[100]);
      this.PeopleGroup = (bool[]) info.GetValue(nameof (PeopleGroup), this.PeopleGroup.GetType());
      this.PeopleGroup = (bool[]) Utils.CopyArray((Array) this.PeopleGroup, (Array) new bool[100]);
      this.ItemGroup = (bool[]) info.GetValue(nameof (ItemGroup), this.ItemGroup.GetType());
      this.ItemGroup = (bool[]) Utils.CopyArray((Array) this.ItemGroup, (Array) new bool[100]);
      this.MaxProd = info.GetInt32(nameof (MaxProd));
      this.IsPort = info.GetBoolean(nameof (IsPort));
      this.IsAirfield = info.GetBoolean(nameof (IsAirfield));
      this.StructuralPts = info.GetInt32(nameof (StructuralPts));
      this.OnDestructLT = info.GetInt32(nameof (OnDestructLT));
      this.OnDestructSpriteNr = info.GetInt32(nameof (OnDestructSpriteNr));
      this.AutoRecoverPts = info.GetInt32(nameof (AutoRecoverPts));
      this.LocTypeGroup = info.GetInt32(nameof (LocTypeGroup));
      this.MinDistance = (int[]) info.GetValue(nameof (MinDistance), this.MinDistance.GetType());
      this.MinDistance = (int[]) Utils.CopyArray((Array) this.MinDistance, (Array) new int[100]);
      this.Buildable = info.GetBoolean(nameof (Buildable));
      this.EPCost = info.GetInt32(nameof (EPCost));
      this.Invincible = info.GetBoolean(nameof (Invincible));
      this.UpgradeNr = info.GetInt32(nameof (UpgradeNr));
      this.DowngradeNr = info.GetInt32(nameof (DowngradeNr));
      this.DowngradeNr = -1;
      this.SupplyCost = info.GetInt32(nameof (SupplyCost));
      this.PPCost = info.GetInt32(nameof (PPCost));
      try
      {
        this.ZOrder = info.GetInt32(nameof (ZOrder));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.ZOrder = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.ExtraGraphic = info.GetInt32(nameof (ExtraGraphic));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.ExtraGraphic = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SmallGraphic = info.GetInt32(nameof (SmallGraphic));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SmallGraphic = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SmallGraphicSpecialMode = info.GetInt32(nameof (SmallGraphicSpecialMode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SmallGraphicSpecialMode = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SmallGraphicSpecialData = info.GetInt32(nameof (SmallGraphicSpecialData));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SmallGraphicSpecialData = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Research = (int[]) info.GetValue(nameof (Research), this.Research.GetType());
        this.VarType = (int[]) info.GetValue(nameof (VarType), this.VarType.GetType());
        this.VarQty = (int[]) info.GetValue(nameof (VarQty), this.VarQty.GetType());
        this.Description = info.GetString(nameof (Description));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut index: i32 =  0;
        do
        {
          this.Research[index] = -1;
          this.VarType[index] = -1;
          this.VarQty[index] = 0;
          this.Description = "";
          index += 1;
        }
        while (index <= 4);
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SlotType = info.GetInt32(nameof (SlotType));
        this.SlotValue = info.GetInt32(nameof (SlotValue));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SlotType = -1;
        this.SlotValue = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.PictureLT = info.GetInt32(nameof (PictureLT));
        this.PictureSprite = info.GetInt32(nameof (PictureSprite));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.PictureLT = -1;
        this.PictureSprite = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.AIPriority = info.GetInt32(nameof (AIPriority));
        this.AIEvent = info.GetInt32(nameof (AIEvent));
        this.AILocEvent = info.GetInt32(nameof (AILocEvent));
        this.AICanBuild = info.GetBoolean(nameof (AICanBuild));
        this.HumanCanBuild = info.GetBoolean(nameof (HumanCanBuild));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.AICanBuild = false;
        this.HumanCanBuild = false;
        this.AIEvent = -1;
        this.AILocEvent = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.AIAfterBuildEvent = info.GetInt32(nameof (AIAfterBuildEvent));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.AIAfterBuildEvent = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.SetPeopleToSlotX = info.GetInt32(nameof (SetPeopleToSlotX));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.SetPeopleToSlotX = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.NoHQ = info.GetBoolean(nameof (NoHQ));
        this.AutoProd = info.GetInt32(nameof (AutoProd));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.NoHQ = false;
        this.AutoProd = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Repairable = info.GetBoolean(nameof (Repairable));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Repairable = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.TopAirStack = info.GetInt32(nameof (TopAirStack));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.TopAirStack = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.Logistical = info.GetInt32(nameof (Logistical));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.Logistical = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.useSmallLabel = info.GetBoolean("UseSmallLabel");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.useSmallLabel = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.isSupplyBase = info.GetBoolean(nameof (isSupplyBase));
        this.maxSupply = info.GetInt32(nameof (maxSupply));
        this.maxFuel = info.GetInt32(nameof (maxFuel));
        this.maxEvacuate = info.GetInt32(nameof (maxEvacuate));
        this.maxDestroy = info.GetInt32(nameof (maxDestroy));
        this.needsCityLevel = info.GetInt32(nameof (needsCityLevel));
        this.cityLevel = info.GetInt32(nameof (cityLevel));
        this.supplyRange = info.GetInt32(nameof (supplyRange));
        this.editorBlock = info.GetBoolean(nameof (editorBlock));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.isSupplyBase = false;
        this.maxSupply = 0;
        this.maxFuel = 0;
        this.maxEvacuate = 0;
        this.maxDestroy = 0;
        this.needsCityLevel = 0;
        this.cityLevel = 0;
        this.supplyRange = 0;
        this.editorBlock = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.isSupplySource = info.GetBoolean(nameof (isSupplySource));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.isSupplySource = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        this.editorBlock = info.GetBoolean(nameof (editorBlock));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        this.editorBlock = false;
        ProjectData.ClearProjectError();
      }
    }

    pub LocationTypeClass(hardcoded: i32)
    {
      this.BuildgroundType = new bool[100];
      this.PeopleGroup = new bool[100];
      this.ItemGroup = new bool[100];
      this.MinDistance = new int[100];
      this.Research = new int[5];
      this.VarType = new int[5];
      this.VarQty = new int[5];
      this.isSupplySource = false;
      this.isSupplyBase = false;
      this.maxSupply = 0;
      this.maxFuel = 0;
      this.maxEvacuate = 0;
      this.maxDestroy = 0;
      this.needsCityLevel = 0;
      this.cityLevel = 0;
      this.supplyRange = 0;
      this.editorBlock = false;
      this.PictureLT = -1;
      this.PictureSprite = -1;
      this.AIEvent = -1;
      this.useSmallLabel = false;
      this.Logistical = 0;
      this.TopAirStack = 0;
      this.NoHQ = false;
      this.AutoProd = -1;
      this.AILocEvent = -1;
      this.SetPeopleToSlotX = -1;
      let mut index1: i32 =  0;
      do
      {
        this.Research[index1] = -1;
        this.VarType[index1] = -1;
        this.VarQty[index1] = 0;
        index1 += 1;
      }
      while (index1 <= 4);
      if (hardcoded == 0)
      {
        this.Name = "Default Loc Type";
        this.OverdrawLTNr = 0;
        this.OverdrawSpriteNr = 0;
        let mut index2: i32 =  0;
        do
        {
          this.BuildgroundType[index2] = false;
          this.PeopleGroup[index2] = false;
          index2 += 1;
        }
        while (index2 <= 19);
        let mut index3: i32 =  0;
        do
        {
          this.ItemGroup[index3] = false;
          index3 += 1;
        }
        while (index3 <= 39);
        this.MaxProd = 1000;
        this.ExtraGraphic = -1;
      }
      this.SlotType = -1;
      this.SlotValue = -1;
      this.AIAfterBuildEvent = -1;
      this.SmallGraphic = -1;
      this.SmallGraphicSpecialMode = -1;
      this.SmallGraphicSpecialData = -1;
    }

    pub LocationTypeClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (LocationTypeClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn Kill()
    {
    }

    pub fn LoadSprites()
    {
    }
  }
}
