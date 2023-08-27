// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LibraryClass
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
  pub class LibraryClass : ISerializable
  {
    pub name: String;
    pub version: i32;
    pub creator: String;
    pub information: String;
    pub lastFileLocation: String;

    pub LibraryClass()
    {
      self.name = "New Lib";
      self.version = 1;
      self.creator = "Unknown".to_owned();
      self.information = "No info";
      self.lastFileLocation = "";
    }

    pub LibraryClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (LibraryClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("name",  self.name);
      info.AddValue("version", self.version);
      info.AddValue("creator",  self.creator);
      info.AddValue("information",  self.information);
      info.AddValue("lastFileLocation",  self.lastFileLocation);
    }

    protected LibraryClass(SerializationInfo info, StreamingContext context)
    {
      self.name = info.GetString(nameof (name));
      self.version = info.GetInt32(nameof (version));
      self.creator = info.GetString(nameof (creator));
      self.information = info.GetString(nameof (information));
      self.lastFileLocation = info.GetString(nameof (lastFileLocation));
    }
  }
}
