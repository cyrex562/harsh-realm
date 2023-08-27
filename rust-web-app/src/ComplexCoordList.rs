// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ComplexCoordList
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class ComplexCoordList
  {
    pub counter: i32;
    pub Coordinate[] coord;
    pub CoordList[] coordList;
    pub maxcounter: i32;

    pub ComplexCoordList()
    {
      this.coord = new Coordinate[50];
      this.coordList = new CoordList[50];
      this.counter = -1;
      this.maxcounter = 49;
    }

    pub fn AddCoord(x: i32, y: i32, map: i32, CoordList tCoordList)
    {
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        this.maxcounter += 49;
        this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
        this.coordList = (CoordList[]) Utils.CopyArray((Array) this.coordList, (Array) new CoordList[this.maxcounter + 1]);
      }
      this.coord[this.counter].x = x;
      this.coord[this.counter].y = y;
      this.coord[this.counter].map = map;
      this.coord[this.counter].onmap = true;
      this.coordList[this.counter] = tCoordList;
    }

    pub fn AddCoord(x: i32, y: i32, map: i32, dat1: i32, dat2: i32, CoordList tCoordList)
    {
      if (DrawMod.TGame.Data.Product > 6 && this.FindSlot(x, y, map) > -1)
        return;
      this += 1.counter;
      if (this.counter > this.maxcounter)
      {
        this.maxcounter += 49;
        this.coord = (Coordinate[]) Utils.CopyArray((Array) this.coord, (Array) new Coordinate[this.maxcounter + 1]);
        this.coordList = (CoordList[]) Utils.CopyArray((Array) this.coordList, (Array) new CoordList[this.maxcounter + 1]);
      }
      this.coord[this.counter].x = x;
      this.coord[this.counter].y = y;
      this.coord[this.counter].map = map;
      this.coord[this.counter].onmap = true;
      this.coord[this.counter].data1 = dat1;
      this.coord[this.counter].data2 = dat2;
      this.coordList[this.counter] = tCoordList;
    }

    pub fn RemoveCoord(nr: i32)
    {
      if (nr < this.counter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  this.counter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
        {
          this.coord[index] = this.coord[index + 1];
          this.coordList[index] = this.coordList[index + 1];
        }
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
  }
}
