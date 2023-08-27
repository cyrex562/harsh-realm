// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ItemList
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
  pub class ItemList : ISerializable
  {
    pub SimpleList list;

    pub ItemList Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (ItemList) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("counter", this.list.Counter);
      if (this.list.Counter <= -1)
        return;
      int[] arySrc1 = (int[]) this.list.Id.Clone();
      int[] arySrc2 = (int[]) this.list.Weight.Clone();
      int[] arySrc3 = (int[]) this.list.Data1.Clone();
      int[] arySrc4 = (int[]) this.list.Data1.Clone();
      int[] numArray1 = (int[]) Utils.CopyArray((Array) arySrc1, (Array) new int[this.list.Counter + 1]);
      int[] numArray2 = (int[]) Utils.CopyArray((Array) arySrc2, (Array) new int[this.list.Counter + 1]);
      int[] numArray3 = (int[]) Utils.CopyArray((Array) arySrc3, (Array) new int[this.list.Counter + 1]);
      int[] numArray4 = (int[]) Utils.CopyArray((Array) arySrc4, (Array) new int[this.list.Counter + 1]);
      info.AddValue("id",  numArray1);
      info.AddValue("weight",  numArray2);
      info.AddValue("data1",  numArray3);
      info.AddValue("data2",  numArray4);
    }

    protected ItemList(SerializationInfo info, StreamingContext context)
    {
      this.list = SimpleList::new();
      let mut int32: i32 =  info.GetInt32("counter");
      int[] numArray1 = new int[int32 + 1];
      int[] numArray2 = new int[int32 + 1];
      int[] numArray3 = new int[int32 + 1];
      int[] numArray4 = new int[int32 + 1];
      if (int32 > -1)
      {
        int[] numArray5 = (int[]) info.GetValue("id", numArray1.GetType());
        int[] numArray6 = (int[]) info.GetValue("weight", numArray2.GetType());
        int[] numArray7 = (int[]) info.GetValue("data1", numArray3.GetType());
        int[] numArray8 = (int[]) info.GetValue("data2", numArray4.GetType());
        this.list.Id = new int[int32 + 1];
        this.list.Weight = new int[int32 + 1];
        this.list.Data1 = new int[int32 + 1];
        this.list.Data2 = new int[int32 + 1];
        this.list.Data3 = new int[int32 + 1];
        this.list.Data4 = new int[int32 + 1];
        this.list.Data5 = new int[int32 + 1];
        this.list.Id = numArray5;
        this.list.Weight = numArray6;
        this.list.Data1 = numArray7;
        this.list.Data2 = numArray8;
      }
      if (int32 < this.list.MaxCounter)
      {
        this.list.Id = (int[]) Utils.CopyArray((Array) this.list.Id, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Weight = (int[]) Utils.CopyArray((Array) this.list.Weight, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Data1 = (int[]) Utils.CopyArray((Array) this.list.Data1, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Data2 = (int[]) Utils.CopyArray((Array) this.list.Data2, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Data3 = (int[]) Utils.CopyArray((Array) this.list.Data3, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Data4 = (int[]) Utils.CopyArray((Array) this.list.Data4, (Array) new int[this.list.MaxCounter + 1]);
        this.list.Data5 = (int[]) Utils.CopyArray((Array) this.list.Data5, (Array) new int[this.list.MaxCounter + 1]);
      }
      else
        this.list.MaxCounter = int32;
      this.list.Counter = int32;
    }

    pub ItemList() => this.list = SimpleList::new();
  }
}
