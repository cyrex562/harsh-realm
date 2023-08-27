// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HexHeightMapData
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub class HexHeightMapData
  {
    pub Coordinate[] neighbourCoord;
    pub neighbourHeight: Vec<i32>;
    pub maxLowerHeight: i32;
    pub lineHeightLevel: Vec<i32>;
    pub maxLines: i32;
    pub cHeight: i32;
    pub HexRiverHeight: Vec<i32>;
    pub highestRiverHeight: i32;
    pub riverHeightApplied: Vec<i32>;
    pub seaHex: bool;

    pub HexHeightMapData( game: GameClass, cx: i32, cy: i32, cmap: i32, bool forInteriorSea)
    {
      this.neighbourCoord = new Coordinate[7];
      this.neighbourHeight = new int[7];
      this.lineHeightLevel = new int[4];
      this.HexRiverHeight = new int[7];
      this.riverHeightApplied = new int[7];
      this.maxLowerHeight = 999;
      bool flag = false;
      if (game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
        flag = true;
      if (flag & forInteriorSea && game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].Interior)
        flag = false;
      this.cHeight = game.Data.MapObj[0].HexObj[cx, cy].HeightLevel;
      if (flag)
        this.cHeight = 9 - this.cHeight;
      this.seaHex = flag;
      let mut tfacing1: i32 =  1;
      Coordinate coordinate;
      do
      {
        coordinate = game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, tfacing1);
        this.neighbourCoord[tfacing1] = coordinate;
        if (coordinate.onmap)
        {
          this.neighbourHeight[tfacing1] = game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel;
          if (game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea & (!forInteriorSea | !game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].Interior))
            this.neighbourHeight[tfacing1] = 9 - this.neighbourHeight[tfacing1];
          if (flag & !game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
            this.neighbourHeight[tfacing1] = 9;
          if (!flag & game.Data.LandscapeTypeObj[game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
            this.neighbourHeight[tfacing1] = 0;
        }
        else
          this.neighbourHeight[tfacing1] = this.cHeight;
        let mut num: i32 =  this.neighbourHeight[tfacing1] - this.cHeight;
        if (num < this.maxLowerHeight)
          this.maxLowerHeight = num;
        tfacing1 += 1;
      }
      while (tfacing1 <= 6);
      this.maxLines = 0;
      if (this.maxLowerHeight <= -4)
        return;
      if (!flag)
      {
        let mut index1: i32 =  1;
        do
        {
          let mut index2: i32 =  game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1 - 1];
          let mut num1: i32 =  0;
          this.riverHeightApplied[index1] = 0;
          if (index2 > -1)
          {
            num1 = game.Data.RiverTypeObj[index2].GetRiverHeight(game, cx, cy, index1 - 1);
            let mut num2: i32 =  index1 + 3;
            if (num2 > 6)
              num2 -= 6;
            let mut tfacing2: i32 =  num2 + 1;
            let mut tfacing3: i32 =  num2 - 1;
            if (tfacing2 > 6)
              tfacing2 = 1;
            if (tfacing3 < 1)
              tfacing3 = 6;
            let mut num3: i32 =  0;
            let mut index3: i32 =  game.Data.MapObj[cmap].HexObj[this.neighbourCoord[index1].x, this.neighbourCoord[index1].y].RiverType[tfacing2 - 1];
            if (index3 > -1 && game.Data.RiverTypeObj[index3].GetRiverHeight(game, this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, tfacing2 - 1) > 0 & this.neighbourHeight[tfacing2] > num3)
              num3 = this.neighbourHeight[tfacing2];
            let mut index4: i32 =  game.Data.MapObj[cmap].HexObj[this.neighbourCoord[index1].x, this.neighbourCoord[index1].y].RiverType[tfacing3 - 1];
            if (index4 > -1 && game.Data.RiverTypeObj[index4].GetRiverHeight(game, this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, tfacing3 - 1) > 0 & this.neighbourHeight[tfacing3] > num3)
              num3 = this.neighbourHeight[tfacing3];
            coordinate = game.HandyFunctionsObj.HexNeighbour(this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, cmap, tfacing2);
            if (coordinate.onmap)
            {
              let mut heightLevel: i32 =  game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel;
              if (heightLevel >= this.neighbourHeight[index1] + 3)
                num1 = 0;
              if (heightLevel >= this.cHeight + 3)
                num1 = 0;
            }
            coordinate = game.HandyFunctionsObj.HexNeighbour(this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, cmap, tfacing3);
            if (coordinate.onmap)
            {
              let mut heightLevel: i32 =  game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel;
              if (heightLevel >= this.neighbourHeight[index1] + 3)
                num1 = 0;
              if (heightLevel >= this.cHeight + 3)
                num1 = 0;
            }
            if (this.neighbourHeight[index1] >= this.cHeight + 3)
              num1 = 0;
            if (num1 > 0 & this.neighbourCoord[index1].onmap & num3 >= this.cHeight)
            {
              let mut num4: i32 =  index1 + 3;
              if (num4 > 6)
                num4 -= 6;
              let mut num5: i32 =  num4 + 1;
              let mut num6: i32 =  num4 - 1;
              if (num5 > 6)
                num5 = 1;
              if (num6 < 1)
                num6 = 6;
              if (num1 > 0)
              {
                let mut index5: i32 =  game.Data.MapObj[cmap].HexObj[this.neighbourCoord[index1].x, this.neighbourCoord[index1].y].RiverType[num5 - 1];
                let mut index6: i32 =  game.Data.MapObj[cmap].HexObj[this.neighbourCoord[index1].x, this.neighbourCoord[index1].y].RiverType[num6 - 1];
                if (index5 > -1 && game.Data.RiverTypeObj[index5].GetRiverHeight(game, this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, num5 - 1) < 1)
                  index5 = -1;
                if (index6 > -1 && game.Data.RiverTypeObj[index6].GetRiverHeight(game, this.neighbourCoord[index1].x, this.neighbourCoord[index1].y, num6 - 1) < 1)
                  index6 = -1;
                if (index5 > -1 | index6 > -1)
                {
                  if (this.neighbourHeight[index1] >= this.cHeight)
                    ;
                  if (this.neighbourHeight[index1] <= this.cHeight)
                    ;
                }
              }
            }
            if (num1 > 0)
            {
              let mut num7: i32 =  this.neighbourHeight[index1];
              this.neighbourHeight[index1] = this.neighbourHeight[index1] >= this.cHeight ? this.cHeight - num1 : this.neighbourHeight[index1] - num1;
              if (this.neighbourHeight[index1] < this.cHeight - 3)
                this.neighbourHeight[index1] = this.cHeight - 3;
              if (this.neighbourHeight[index1] < 0)
                this.neighbourHeight[index1] = 0;
              let mut num8: i32 =  this.neighbourHeight[index1] - this.cHeight;
              if (num8 < this.maxLowerHeight)
                this.maxLowerHeight = num8;
              if (num7 > this.neighbourHeight[index1])
                this.riverHeightApplied[index1] = 1;
            }
          }
          this.HexRiverHeight[index1] = num1;
          if (num1 > this.highestRiverHeight)
            this.highestRiverHeight = num1;
          index1 += 1;
        }
        while (index1 <= 6);
      }
      if (this.maxLowerHeight >= 0)
        return;
      if (this.maxLowerHeight == -1)
      {
        this.maxLines = 1;
        this.lineHeightLevel[1] = this.cHeight - 1;
      }
      if (this.maxLowerHeight == -2)
      {
        this.maxLines = 2;
        this.lineHeightLevel[1] = this.cHeight - 2;
        this.lineHeightLevel[2] = this.cHeight - 1;
      }
      if (this.maxLowerHeight != -3)
        return;
      this.maxLines = 3;
      this.lineHeightLevel[1] = this.cHeight - 3;
      this.lineHeightLevel[2] = this.cHeight - 2;
      this.lineHeightLevel[3] = this.cHeight - 1;
    }
  }
}
