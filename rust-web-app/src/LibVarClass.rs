// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LibVarClass
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
  pub class LibVarClass : ISerializable
  {
    pub NewEnums.LibVarType type;
    pub LibIdClass libId;
    pub name: String;
    pub NewEnums.LibVarValueType valueType;
    pub LibIdClass instanceId;
    pub value: i32;
    pub valueText: String;
    pub information: String;

    pub LibVarClass(tlibSlot: i32)
    {
      self.type = NewEnums.LibVarType.General;
      self.libId = LibIdClass::new();
      self.libId.libSlot = tlibSlot;
      self.libId.id = -1;
      self.valueType = NewEnums.LibVarValueType.Number;
      self.value = 0;
      self.valueText = "";
      self.instanceId = LibIdClass::new();
      self.instanceId.libSlot = -1;
      self.instanceId.id = -1;
    }

    pub LibVarClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (LibVarClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub GetValue: String( Data: DataClass, bool ForEvent = false)
    {
      str: String = self.value.ToString();
      if (self.valueType == NewEnums.LibVarValueType.Number)
        return self.value.ToString();
      if (self.valueType == NewEnums.LibVarValueType.YesNo)
        return !ForEvent ? (self.value != 1 ? "No" : "Yes") : (self.value != 1 ? "0" : "1");
      if (self.valueType == NewEnums.LibVarValueType.Text)
        return self.valueText;
      if (self.valueType == NewEnums.LibVarValueType.DateString)
      {
        Left: String = self.valueText;
        if (ForEvent)
        {
          if (Operators.CompareString(Left, "", false) == 0)
            Left = "0".to_owned();
        }
        else if (Operators.CompareString(Left, "", false) == 0)
          Left = "-not set-";
        return Left;
      }
      if (self.valueType == NewEnums.LibVarValueType.HistoricalUnitId | self.valueType == NewEnums.LibVarValueType.HistoricalUnitModelId)
        str = !(self.value > -1 & self.value <= Data.HistoricalUnitCounter) ? (self.value <= Data.HistoricalUnitCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.HistoricalUnitObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.OfficerId)
        str = !(self.value > -1 & self.value <= Data.HistoricalUnitCounter) ? (self.value <= Data.HistoricalUnitCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.HistoricalUnitObj[self.value].CommanderName;
      if (self.valueType == NewEnums.LibVarValueType.LandscapeId)
        str = !(self.value > -1 & self.value <= Data.LandscapeTypeCounter) ? (self.value <= Data.LandscapeTypeCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.LandscapeTypeObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.EventPicId)
        str = !(self.value > -1 & self.value <= Data.EventPicCounter) ? (self.value <= Data.EventPicCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.EventPicName[self.value];
      if (self.valueType == NewEnums.LibVarValueType.SmallGfxId)
        str = !(self.value > -1 & self.value <= Data.SmallPicCounter) ? (self.value <= Data.SmallPicCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.SmallPicName[self.value];
      if (self.valueType == NewEnums.LibVarValueType.PeopleId)
        str = !(self.value > -1 & self.value <= Data.PeopleCounter) ? (self.value <= Data.PeopleCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.PeopleObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.RiverId)
        str = !(self.value > -1 & self.value <= Data.RiverTypeCounter) ? (self.value <= Data.RiverTypeCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.RiverTypeObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.RoadId)
        str = !(self.value > -1 & self.value <= Data.RoadTypeCounter) ? (self.value <= Data.RoadTypeCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.RoadTypeObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.SFTypeId)
        str = !(self.value > -1 & self.value <= Data.SFTypeCounter) ? (self.value <= Data.SFTypeCounter ? (self.value != -1 ? "Invalid value" : "None/All") : "Non-existing!") : Data.SFTypeObj[self.value].Name;
      if (self.valueType == NewEnums.LibVarValueType.YesNo)
        str = self.value <= 0 ? "No" : "Yes".to_owned();
      if (ForEvent && self.value == -1)
        return "-1";
      return ForEvent ? Conversions.ToString(self.value) : str;
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("type",  self.type);
      info.AddValue("libId",  self.libId);
      info.AddValue("name",  self.name);
      info.AddValue("valueType",  self.valueType);
      info.AddValue("value", self.value);
      info.AddValue("valueText",  self.valueText);
      if (self.libId.id == -1)
        info.AddValue("information",  self.information);
      info.AddValue("instanceId",  self.instanceId);
    }

    protected LibVarClass(SerializationInfo info, StreamingContext context)
    {
      self.type = (NewEnums.LibVarType) info.GetInt32(nameof (type));
      self.libId = LibIdClass::new();
      self.libId = (LibIdClass) info.GetValue(nameof (libId), self.libId.GetType());
      self.name = info.GetString(nameof (name));
      self.valueType = (NewEnums.LibVarValueType) info.GetInt32(nameof (valueType));
      self.value = info.GetInt32(nameof (value));
      self.valueText = info.GetString(nameof (valueText));
      if (self.libId.id == -1)
        self.information = info.GetString(nameof (information));
      try
      {
        self.instanceId = LibIdClass::new();
        self.instanceId = (LibIdClass) info.GetValue(nameof (instanceId), self.instanceId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.instanceId = LibIdClass::new();
        self.instanceId.libSlot = -1;
        self.instanceId.id = -1;
        ProjectData.ClearProjectError();
      }
    }
  }
}
