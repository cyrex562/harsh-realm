// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ItemTypeClass
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
  pub class ItemTypeClass : ISerializable
  {
    pub Name: String;
    pub IsSupply: bool;
    pub ItemGroup: i32;
    pub GameSlotsNeeded: Vec<i32>;
    pub GameSlotsNeededQty: Vec<i32>;
    pub RegimeSlotsNeeded: Vec<i32>;
    pub RegimeSlotsNeededQty: Vec<i32>;
    pub RegimeSlotsCost: Vec<i32>;
    pub RegimeSlotsCostQty: Vec<i32>;
    pub ProdWeight: i32;
    pub PeopleGroup: Vec<bool>;
    pub IsResPt: bool;
    pub IsSFType: i32;
    pub ResFieldNeeded: Vec<i32>;
    pub Multiplier: i32;
    pub Blocks: i32;
    pub IsRegimeSlot: i32;
    pub UseSFType: i32;
    pub RegimeSpecific: i32;
    pub UseProdMod: i32;
    pub XpMod: i32;
    pub MorMod: i32;
    pub PeopleMod: i32;
    pub MoveTypeMod: i32;

    pub ItemTypeClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (ItemTypeClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("IsSupply", this.IsSupply);
      info.AddValue("ItemGroup", this.ItemGroup);
      info.AddValue("GameSlotsNeeded",  this.GameSlotsNeeded);
      info.AddValue("GameSlotsNeededQty",  this.GameSlotsNeededQty);
      info.AddValue("RegimeSlotsNeeded",  this.RegimeSlotsNeeded);
      info.AddValue("RegimeSlotsNeededQty",  this.RegimeSlotsNeededQty);
      info.AddValue("RegimeSlotsCost",  this.RegimeSlotsCost);
      info.AddValue("RegimeSlotsCostQty",  this.RegimeSlotsCostQty);
      info.AddValue("ProdWeight", this.ProdWeight);
      info.AddValue("PeopleGroup",  this.PeopleGroup);
      info.AddValue("IsResPt", this.IsResPt);
      info.AddValue("ResFieldNeeded",  this.ResFieldNeeded);
      info.AddValue("Multiplier", this.Multiplier);
      info.AddValue("IsSFType", this.IsSFType);
      info.AddValue("Blocks", this.Blocks);
      info.AddValue("IsRegimeSlot", this.IsRegimeSlot);
      info.AddValue("UseSFType", this.UseSFType);
      info.AddValue("XpMod", this.XpMod);
      info.AddValue("MorMod", this.MorMod);
      info.AddValue("PeopleMod", this.PeopleMod);
      info.AddValue("MoveTypeMod", this.MoveTypeMod);
      info.AddValue("RegimeSpecific", this.RegimeSpecific);
      info.AddValue("UseProdMod", this.UseProdMod);
    }

    protected ItemTypeClass(SerializationInfo info, StreamingContext context)
    {
      this.GameSlotsNeeded = new int[5];
      this.GameSlotsNeededQty = new int[5];
      this.RegimeSlotsNeeded = new int[5];
      this.RegimeSlotsNeededQty = new int[5];
      this.RegimeSlotsCost = new int[5];
      this.RegimeSlotsCostQty = new int[5];
      this.PeopleGroup = new bool[100];
      this.ResFieldNeeded = new int[5];
      this.Name = info.GetString(nameof (Name));
      this.IsSupply = info.GetBoolean(nameof (IsSupply));
      this.ItemGroup = info.GetInt32(nameof (ItemGroup));
      this.GameSlotsNeeded = (int[]) info.GetValue(nameof (GameSlotsNeeded), this.GameSlotsNeeded.GetType());
      this.GameSlotsNeededQty = (int[]) info.GetValue(nameof (GameSlotsNeededQty), this.GameSlotsNeededQty.GetType());
      this.RegimeSlotsNeeded = (int[]) info.GetValue(nameof (RegimeSlotsNeeded), this.RegimeSlotsNeeded.GetType());
      this.RegimeSlotsNeededQty = (int[]) info.GetValue(nameof (RegimeSlotsNeededQty), this.RegimeSlotsNeededQty.GetType());
      this.ProdWeight = info.GetInt32(nameof (ProdWeight));
      this.PeopleGroup = (bool[]) info.GetValue(nameof (PeopleGroup), this.PeopleGroup.GetType());
      this.PeopleGroup = (bool[]) Utils.CopyArray((Array) this.PeopleGroup, (Array) new bool[100]);
      this.IsResPt = info.GetBoolean(nameof (IsResPt));
      this.ResFieldNeeded = (int[]) info.GetValue(nameof (ResFieldNeeded), this.ResFieldNeeded.GetType());
      this.Multiplier = info.GetInt32(nameof (Multiplier));
      this.IsSFType = info.GetInt32(nameof (IsSFType));
      this.Blocks = info.GetInt32(nameof (Blocks));
      this.IsRegimeSlot = info.GetInt32(nameof (IsRegimeSlot));
      if (DrawMod.TGame.Data.Version > 100)
      {
        this.RegimeSlotsCost = (int[]) info.GetValue(nameof (RegimeSlotsCost), this.RegimeSlotsCost.GetType());
        this.RegimeSlotsCostQty = (int[]) info.GetValue(nameof (RegimeSlotsCostQty), this.RegimeSlotsCostQty.GetType());
      }
      else
      {
        let mut index: i32 =  0;
        do
        {
          this.RegimeSlotsCost[index] = -1;
          this.RegimeSlotsCostQty[index] = 0;
          index += 1;
        }
        while (index <= 4);
      }
      if (DrawMod.TGame.Data.Version < 158)
      {
        try
        {
          this.UseSFType = info.GetInt32(nameof (UseSFType));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UseSFType = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.XpMod = info.GetInt32(nameof (XpMod));
          this.MorMod = info.GetInt32(nameof (MorMod));
          this.PeopleMod = info.GetInt32(nameof (PeopleMod));
          this.MoveTypeMod = info.GetInt32(nameof (MoveTypeMod));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.MoveTypeMod = -1;
          this.PeopleMod = -1;
          this.MorMod = 0;
          this.XpMod = 0;
          ProjectData.ClearProjectError();
        }
        this.RegimeSpecific = -1;
      }
      else
      {
        this.UseSFType = info.GetInt32(nameof (UseSFType));
        this.XpMod = info.GetInt32(nameof (XpMod));
        this.MorMod = info.GetInt32(nameof (MorMod));
        this.PeopleMod = info.GetInt32(nameof (PeopleMod));
        this.MoveTypeMod = info.GetInt32(nameof (MoveTypeMod));
        try
        {
          this.RegimeSpecific = info.GetInt32(nameof (RegimeSpecific));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.RegimeSpecific = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          this.UseProdMod = info.GetInt32(nameof (UseProdMod));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.UseProdMod = 1;
          ProjectData.ClearProjectError();
        }
      }
    }

    pub ItemTypeClass(hardcoded: i32)
    {
      this.GameSlotsNeeded = new int[5];
      this.GameSlotsNeededQty = new int[5];
      this.RegimeSlotsNeeded = new int[5];
      this.RegimeSlotsNeededQty = new int[5];
      this.RegimeSlotsCost = new int[5];
      this.RegimeSlotsCostQty = new int[5];
      this.PeopleGroup = new bool[100];
      this.ResFieldNeeded = new int[5];
      this.Name = "Default Item Type";
      let mut index1: i32 =  0;
      do
      {
        this.GameSlotsNeeded[index1] = -1;
        this.GameSlotsNeededQty[index1] = -1;
        this.RegimeSlotsNeeded[index1] = -1;
        this.RegimeSlotsNeededQty[index1] = -1;
        this.ResFieldNeeded[index1] = -1;
        this.RegimeSlotsCost[index1] = -1;
        this.RegimeSlotsCostQty[index1] = 0;
        index1 += 1;
      }
      while (index1 <= 4);
      let mut index2: i32 =  0;
      do
      {
        this.PeopleGroup[index2] = false;
        index2 += 1;
      }
      while (index2 <= 19);
      this.ProdWeight = 1;
      this.IsResPt = false;
      this.IsSFType = -1;
      this.UseProdMod = 1;
      this.Blocks = -1;
      this.Multiplier = 1;
      this.UseSFType = -1;
      this.MoveTypeMod = -1;
      this.PeopleMod = -1;
      this.MorMod = 0;
      this.XpMod = 0;
      this.RegimeSpecific = -1;
      this.IsRegimeSlot = -1;
    }

    pub fn Kill()
    {
    }

    pub fn LoadSprites()
    {
    }
  }
}
