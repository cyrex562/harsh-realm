// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.AreaClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;

namespace WindowsApplication1
{
  [Serializable]
  pub class AreaClass : ISerializable
  {
    pub Name: String;
    pub Slot: i32;
    pub Code: i32;
    pub ID: i32;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  this.Name);
      info.AddValue("Slot", this.Slot);
      info.AddValue("Code", this.Code);
      info.AddValue("ID", this.ID);
    }

    protected AreaClass(SerializationInfo info, StreamingContext context)
    {
      this.Name = info.GetString(nameof (Name));
      this.Slot = info.GetInt32(nameof (Slot));
      this.Code = info.GetInt32(nameof (Code));
      this.ID = info.GetInt32(nameof (ID));
    }

    pub AreaClass()
    {
      this.Name = "New Area";
      this.Slot = -1;
      this.Code = -1;
    }

    pub AreaClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (AreaClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }
  }
}
