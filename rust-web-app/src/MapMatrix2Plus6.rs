// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapMatrix2Plus6
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
  pub class MapMatrix2Plus6 : ISerializable
  {
    pub Width: i32;
    pub Height: i32;
    pub bool[,,] Value;
    pub int[,,] Value2;

    pub MapMatrix2Plus6 Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (MapMatrix2Plus6) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Width", this.Width);
      info.AddValue("Height", this.Height);
      info.AddValue("Value",  this.Value);
      info.AddValue("Value2",  this.Value2);
    }

    protected MapMatrix2Plus6(SerializationInfo info, StreamingContext context)
    {
      this.Value = new bool[1, 1, 6];
      this.Value2 = new int[1, 1, 6];
      this.Width = info.GetInt32(nameof (Width));
      this.Height = info.GetInt32(nameof (Height));
      this.Value = new bool[this.Width + 1, this.Height + 1, 6];
      this.Value = (bool[,,]) info.GetValue(nameof (Value), this.Value.GetType());
      this.Value2 = (int[,,]) info.GetValue(nameof (Value2), this.Value2.GetType());
    }

    pub MapMatrix2Plus6(w: i32, h: i32)
    {
      this.Value = new bool[1, 1, 6];
      this.Value2 = new int[1, 1, 6];
      this.Width = w;
      this.Height = h;
      this.Value = new bool[this.Width + 1, this.Height + 1, 6];
      this.Value2 = new int[this.Width + 1, this.Height + 1, 6];
    }
  }
}
