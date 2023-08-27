// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapMatrix2Coordinate
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
  pub class MapMatrix2Coordinate : ISerializable
  {
    pub Width: i32;
    pub Height: i32;
    pub Coordinate[,] Value;

    pub MapMatrix2Coordinate Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (MapMatrix2Coordinate) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Width", this.Width);
      info.AddValue("Height", this.Height);
      if (this.Width <= -1)
        return;
      numArray1: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray2: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray3: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray4: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray5: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      bool[,] flagArray = new bool[this.Width + 1, this.Height + 1];
      let mut width: i32 =  this.Width;
      for (let mut index1: i32 =  0; index1 <= width; index1 += 1)
      {
        let mut height: i32 =  this.Height;
        for (let mut index2: i32 =  0; index2 <= height; index2 += 1)
        {
          numArray1[index1, index2] = this.Value[index1, index2].x;
          numArray2[index1, index2] = this.Value[index1, index2].y;
          numArray3[index1, index2] = this.Value[index1, index2].map;
          numArray4[index1, index2] = this.Value[index1, index2].data1;
          numArray5[index1, index2] = this.Value[index1, index2].data2;
          flagArray[index1, index2] = this.Value[index1, index2].onmap;
        }
      }
      info.AddValue("t1",  numArray1);
      info.AddValue("t2",  numArray2);
      info.AddValue("t3",  numArray3);
      info.AddValue("t4",  numArray4);
      info.AddValue("t5",  numArray5);
      info.AddValue("t6",  flagArray);
    }

    protected MapMatrix2Coordinate(SerializationInfo info, StreamingContext context)
    {
      this.Value = new Coordinate[1, 1];
      this.Width = info.GetInt32(nameof (Width));
      this.Height = info.GetInt32(nameof (Height));
      this.Value = new Coordinate[this.Width + 1, this.Height + 1];
      if (this.Width <= 0)
        return;
      numArray1: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray2: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray3: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray4: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      numArray5: Vec<i32> = new int[this.Width + 1, this.Height + 1];
      bool[,] flagArray1 = new bool[this.Width + 1, this.Height + 1];
      if (this.Width <= -1)
        return;
      numArray6: Vec<i32> = (int[,]) info.GetValue("t1", numArray1.GetType());
      numArray7: Vec<i32> = (int[,]) info.GetValue("t2", numArray2.GetType());
      numArray8: Vec<i32> = (int[,]) info.GetValue("t3", numArray3.GetType());
      numArray9: Vec<i32> = (int[,]) info.GetValue("t4", numArray4.GetType());
      numArray10: Vec<i32> = (int[,]) info.GetValue("t5", numArray5.GetType());
      bool[,] flagArray2 = (bool[,]) info.GetValue("t6", flagArray1.GetType());
      let mut width: i32 =  this.Width;
      for (let mut index1: i32 =  0; index1 <= width; index1 += 1)
      {
        let mut height: i32 =  this.Height;
        for (let mut index2: i32 =  0; index2 <= height; index2 += 1)
        {
          this.Value[index1, index2].x = numArray6[index1, index2];
          this.Value[index1, index2].y = numArray7[index1, index2];
          this.Value[index1, index2].map = numArray8[index1, index2];
          this.Value[index1, index2].data1 = numArray9[index1, index2];
          this.Value[index1, index2].data2 = numArray10[index1, index2];
          this.Value[index1, index2].onmap = flagArray2[index1, index2];
        }
      }
    }

    pub MapMatrix2Coordinate(w: i32, h: i32)
    {
      this.Value = new Coordinate[1, 1];
      this.Width = w;
      this.Height = h;
      this.Value = new Coordinate[this.Width + 1, this.Height + 1];
    }
  }
}
