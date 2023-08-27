// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CoordList
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
  pub class CoordList : ISerializable
  {
    pub counter: i32;
    pub Coordinate[] coord;
    pub active: bool;
    pub maxcounter: i32;
     bool noPresized;

    pub CoordList()
    {
      this.coord = new Coordinate[1000];
      this.noPresized = false;
      this.counter = -1;
      this.active = false;
      this.maxcounter = 999;
    }

    pub CoordList(whatEverBigCounter9999: i32)
    {
      this.coord = new Coordinate[1000];
      this.noPresized = false;
      this.counter = -1;
      this.active = false;
      this.coord = new Coordinate[whatEverBigCounter9999 + 1];
      this.maxcounter = whatEverBigCounter9999;
    }

    pub CoordList(bool tnoPresized)
    {
      this.coord = new Coordinate[1000];
      this.noPresized = false;
      this.counter = -1;
      this.noPresized = tnoPresized;
      this.active = false;
      this.maxcounter = 0;
      this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[1]);
    }

    pub fn AddCoord(x: i32, y: i32, map: i32)
    {
      this.active = true;
      this += 1.counter;
      if (this.counter >= this.maxcounter)
      {
        if (this.noPresized)
          this.maxcounter += 10;
        else
          this.maxcounter += 1000;
        this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
      }
      this.coord[this.counter].x = x;
      this.coord[this.counter].y = y;
      this.coord[this.counter].map = map;
      this.coord[this.counter].onmap = true;
    }

    pub fn AddCoord(x: i32, y: i32, map: i32, dat1: i32, dat2: i32)
    {
      this.active = true;
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        if (this.noPresized)
          this.maxcounter += 10;
        else
          this.maxcounter += 1000;
        this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
      }
      this.coord[this.counter].x = x;
      this.coord[this.counter].y = y;
      this.coord[this.counter].map = map;
      this.coord[this.counter].onmap = true;
      this.coord[this.counter].data1 = dat1;
      this.coord[this.counter].data2 = dat2;
    }

    pub fn AddList(ref CoordList listy)
    {
      let mut counter: i32 =  listy.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (!this.Exists(listy.coord[index].x, listy.coord[index].y, listy.coord[index].map))
          this.AddCoord(listy.coord[index].x, listy.coord[index].y, listy.coord[index].map);
      }
    }

    pub fn DeActivate() => this.active = false;

    pub fn RemoveCoord(nr: i32)
    {
      if (nr < this.counter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.counter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          this.coord[index] = this.coord[index + 1];
      }
      --this.counter;
      if (this.counter < -1)
        this.counter = -1;
      this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
    }

    pub fn RemoveCoordQuick(nr: i32)
    {
      if (nr < this.counter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.counter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          this.coord[index] = this.coord[index + 1];
      }
      --this.counter;
    }

    pub Exists: bool(x: i32, y: i32, map: i32)
    {
      if (this.counter == -1)
        return false;
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        if (this.coord[index].x == x & this.coord[index].y == y & this.coord[index].map == map)
          return true;
      }
      return false;
    }

    pub fn FindSlot(x: i32, y: i32, map: i32) -> i32
    {
      if (this.counter == -1)
        return -1;
      let mut counter: i32 =  this.counter;
      for (let mut slot: i32 =  0; slot <= counter; slot += 1)
      {
        if (this.coord[slot].x == x & this.coord[slot].y == y & this.coord[slot].map == map)
          return slot;
      }
      return -1;
    }

    pub fn FindSlot(x: i32, y: i32, map: i32, tdata1: i32, tdata2: i32) -> i32
    {
      if (this.counter == -1)
        return -1;
      let mut counter: i32 =  this.counter;
      for (let mut slot: i32 =  0; slot <= counter; slot += 1)
      {
        if ((this.coord[slot].x == x | x == -1) & (this.coord[slot].y == y | y == -1) && this.coord[slot].data1 == tdata1 & this.coord[slot].data2 == tdata2)
          return slot;
      }
      return -1;
    }

    pub CoordList Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (CoordList) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("counter", this.counter);
      if (this.counter <= -1)
        return;
      int[] numArray1 = new int[this.counter + 1];
      int[] numArray2 = new int[this.counter + 1];
      int[] numArray3 = new int[this.counter + 1];
      let mut counter: i32 =  this.counter;
      for (let mut index: i32 =  0; index <= counter; index += 1)
      {
        numArray1[index] = this.coord[index].x;
        numArray2[index] = this.coord[index].y;
        numArray3[index] = this.coord[index].map;
      }
      info.AddValue("t1",  numArray1);
      info.AddValue("t2",  numArray2);
      info.AddValue("t3",  numArray3);
    }

    protected CoordList(SerializationInfo info, StreamingContext context)
    {
      this.coord = new Coordinate[1000];
      this.noPresized = false;
      let mut int32: i32 =  info.GetInt32(nameof (counter));
      int[] numArray1 = new int[int32 + 1];
      int[] numArray2 = new int[int32 + 1];
      int[] numArray3 = new int[int32 + 1];
      if (int32 <= -1)
        return;
      int[] numArray4 = (int[]) info.GetValue("t1", numArray1.GetType());
      int[] numArray5 = (int[]) info.GetValue("t2", numArray2.GetType());
      int[] numArray6 = (int[]) info.GetValue("t3", numArray3.GetType());
      let mut num: i32 =  int32;
      for (let mut index: i32 =  0; index <= num; index += 1)
        this.AddCoord(numArray4[index], numArray5[index], numArray6[index]);
    }
  }
}
