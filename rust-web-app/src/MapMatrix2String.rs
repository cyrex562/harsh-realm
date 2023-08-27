// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapMatrix2String
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
  pub class MapMatrix2String : ISerializable
  {
    pub Width: i32;
    pub Height: i32;
    pub string[,] Value;

    pub MapMatrix2 Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (MapMatrix2) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Width", this.Width);
      info.AddValue("Height", this.Height);
      info.AddValue("Value",  this.Value);
    }

    protected MapMatrix2String(SerializationInfo info, StreamingContext context)
    {
      this.Value = new string[1, 1];
      this.Width = info.GetInt32(nameof (Width));
      this.Height = info.GetInt32(nameof (Height));
      this.Value = new string[this.Width + 1, this.Height + 1];
      this.Value = (string[,]) info.GetValue(nameof (Value), this.Value.GetType());
    }

    pub MapMatrix2String(w: i32, h: i32)
    {
      this.Value = new string[1, 1];
      this.Width = w;
      this.Height = h;
      this.Value = new string[this.Width + 1, this.Height + 1];
    }
  }
}
