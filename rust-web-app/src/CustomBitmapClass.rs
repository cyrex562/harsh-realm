// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CustomBitmapClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.Drawing.Text;
// usingSystem.Runtime.InteropServices;
// usingSystem.Threading;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class CustomBitmapClass
  {
     game: GameClass;
     Graphics g2;
     Graphics g3;
     tmpbmp2: Bitmap;
     tmpbmp2b: Bitmap;
     tmpbmp3: Bitmap;
    pub stdFont1: Font;
    pub stdFont2: Font;
    pub Stdfont3: Font;
    pub Stdfont4: Font;
    pub tempHex: Bitmap;
    pub tempHexSmall: Bitmap;
    pub tempHexMed: Bitmap;
    pub tempHexBig: Bitmap;
    pub strId123slot: i32;
    pub strId143slot: i32;
    pub strId275slot: i32;
    pub strId288slot: i32;
    pub strId534slot: i32;
    pub cache_t6: i32;
    pub cache_tad: i32;
    pub cache_tat: i32;
    pub cache_rad: i32;
    pub cacheDipClear: Vec<i32>;
    pub cacheZoneRecon: Vec<i32>;
    pub slotCulture: i32;
    pub slotAssets: i32;
    pub slotAir: i32;
    pub slotAssetTypes: i32;
    pub slotZones: i32;
    pub slotPaid: i32;
    pub slotAssetLog: i32;
    pub slotConstruction: i32;
    pub slotPerks: i32;
    pub slotHexNames: i32;
    pub slotItemType: i32;
    pub slotAssetPresentation: i32;
    pub slotOrigDetail: i32;
    pub slotDetail: i32;
    pub slotDetailPreview: i32;
    pub slotKeyReplace: i32;
    pub miniMapPredrawnCache: Bitmap;

    pub CustomBitmapClass(tgame: GameClass)
    {
      this.cacheDipClear = new int[2, 2];
      this.cacheZoneRecon = new int[2, 2];
      this.slotCulture = -1;
      this.game = tgame;
      this.tmpbmp2 = new Bitmap(38, 38, PixelFormat.Format32bppPArgb);
      this.tmpbmp2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      this.tmpbmp2b = new Bitmap(76, 76, PixelFormat.Format32bppPArgb);
      this.tmpbmp2b.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      this.g2 = Graphics.FromImage((Image) this.tmpbmp2);
      this.tmpbmp3 = new Bitmap(64, 48, PixelFormat.Format32bppPArgb);
      this.tmpbmp3.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      this.g3 = Graphics.FromImage((Image) this.tmpbmp3);
      this.stdFont1 = !this.game.IsWin10 ? Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel) : Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.stdFont2 = Font::new(this.game.FontCol.Families[1], 9f, FontStyle.Regular, GraphicsUnit.Pixel);
      this.Stdfont3 = Font::new(this.game.FontCol.Families[1], 16f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.Stdfont4 = Font::new(this.game.FontCol.Families[1], 14f, FontStyle.Bold, GraphicsUnit.Pixel);
      this.strId123slot = -1;
      this.slotCulture = -1;
    }

    pub void DrawHeightMapLate(
      ref Graphics useg,
      cx: i32,
      cy: i32,
      cmap: i32,
      tx: i32,
      ty: i32,
      Zoom: i32)
    {
      if (!this.game.AllowHeightMap)
        return;
      bool flag = false;
      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
        flag = true;
      float[] numArray1 = new float[10];
      float[] numArray2 = new float[10];
      float[] numArray3 = new float[10];
      int[] numArray4 = new int[10];
      width: i32;
      height: i32;
      switch (Zoom)
      {
        case -1:
          width = 32;
          height = 24;
          break;
        case 0:
          width = 64;
          height = 48;
          break;
        default:
          width = 128;
          height = 96;
          break;
      }
      if (((flag ? 1 : 0) & 0) != 0)
      {
        numArray4[0] = 90;
        numArray4[1] = 80;
        numArray4[2] = 70;
        numArray4[3] = 60;
        numArray4[4] = 50;
        numArray4[5] = 40;
        numArray4[6] = 30;
        numArray4[7] = 20;
        numArray4[8] = 10;
        numArray4[9] = 0;
        let mut index1: i32 =  -1;
        if ( this.game.Data.RuleVar[416] > 0.0)
          index1 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[416]));
        if (index1 > -1)
        {
          let mut length: i32 =  this.game.Data.StringListObj[index1].Length;
          for (let mut index2: i32 =  0; index2 <= length; index2 += 1)
          {
            let mut index3: i32 =  9 - index2;
            numArray1[index3] =  (Conversion.Val(this.game.Data.StringListObj[index1].Data[index2, 0]) /  byte.MaxValue);
            numArray2[index3] =  (Conversion.Val(this.game.Data.StringListObj[index1].Data[index2, 1]) /  byte.MaxValue);
            numArray3[index3] =  (Conversion.Val(this.game.Data.StringListObj[index1].Data[index2, 2]) /  byte.MaxValue);
            numArray4[index3] = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[index1].Data[index2, 3]));
          }
        }
        let mut tfacing: i32 =  1;
        do
        {
          Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
          if (coordinate.onmap && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
          {
            let mut index4: i32 =  9 - this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel;
            if (numArray4[index4] > 0)
            {
              let mut num1: i32 =  tfacing - 1;
              let mut num2: i32 =  11;
              ref Graphics local1 = ref useg;
              bitmap1: Bitmap = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW1, Zoom);
              ref local2: Bitmap = ref bitmap1;
              Rectangle srcrect = Rectangle::new(num1 * width, num2 * height, width, height);
              Rectangle destrect = Rectangle::new(tx, ty, width, height);
              double r =  numArray1[index4];
              double g =  numArray2[index4];
              double b =  numArray3[index4];
              double a =  ( numArray4[index4] /  byte.MaxValue);
              bitmap2: Bitmap = (Bitmap) null;
              ref local3: Bitmap = ref bitmap2;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local1, ref local2, srcrect, destrect,  r,  g,  b,  a, ref local3);
            }
          }
          tfacing += 1;
        }
        while (tfacing <= 6);
      }
      else
      {
        let mut tfacing: i32 =  1;
        do
        {
          Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
          if (coordinate.onmap)
          {
            let mut num3: i32 =  tfacing + 3;
            if (num3 > 6)
              num3 -= 6;
            if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RiverType[num3 - 1] == 5)
            {
              let mut num4: i32 =  tfacing - 1;
              let mut num5: i32 =  11;
              ref Graphics local4 = ref useg;
              bitmap3: Bitmap = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW1, Zoom);
              ref local5: Bitmap = ref bitmap3;
              Rectangle srcrect = Rectangle::new(num4 * width, num5 * height, width, height);
              Rectangle destrect = Rectangle::new(tx, ty, width, height);
              bitmap4: Bitmap = (Bitmap) null;
              ref local6: Bitmap = ref bitmap4;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local4, ref local5, srcrect, destrect, 0.0f, 0.0f, 0.0f, 0.25f, ref local6);
            }
          }
          tfacing += 1;
        }
        while (tfacing <= 6);
      }
    }

    pub void DrawHeightMap(
      ref Graphics useg,
      cx: i32,
      cy: i32,
      cmap: i32,
      tx: i32,
      ty: i32,
      Zoom: i32,
      bool forInteriorSea,
      ref gBitmap: Bitmap = null)
    {
      if (!this.game.AllowHeightMap)
        return;
      bool flag1 = true;
      bool flag2 = false;
      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
        flag2 = true;
      if (flag2 & forInteriorSea && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].Interior)
        flag2 = false;
      HexHeightMapData hexHeightMapData = new HexHeightMapData(ref this.game, cx, cy, cmap, forInteriorSea);
      int[] numArray1 = new int[7];
      int[] numArray2 = new int[7];
      int[] numArray3 = new int[7];
      int[] numArray4 = new int[7];
      switch (Zoom)
      {
        case -1:
          numArray1[0] = 8;
          numArray2[0] = 4;
          numArray3[0] = 16;
          numArray4[0] = 16;
          numArray1[1] = 8;
          numArray2[1] = 0;
          numArray3[1] = 16;
          numArray4[1] = 4;
          numArray1[2] = 24;
          numArray2[2] = 0;
          numArray3[2] = 8;
          numArray4[2] = 12;
          numArray1[3] = 24;
          numArray2[3] = 12;
          numArray3[3] = 8;
          numArray4[3] = 12;
          numArray1[4] = 8;
          numArray2[4] = 20;
          numArray3[4] = 16;
          numArray4[4] = 4;
          numArray1[5] = 0;
          numArray2[5] = 12;
          numArray3[5] = 8;
          numArray4[5] = 12;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 8;
          numArray4[6] = 12;
          break;
        case 0:
          numArray1[0] = 16;
          numArray2[0] = 8;
          numArray3[0] = 32;
          numArray4[0] = 32;
          numArray1[1] = 16;
          numArray2[1] = 0;
          numArray3[1] = 32;
          numArray4[1] = 8;
          numArray1[2] = 48;
          numArray2[2] = 0;
          numArray3[2] = 16;
          numArray4[2] = 24;
          numArray1[3] = 48;
          numArray2[3] = 24;
          numArray3[3] = 16;
          numArray4[3] = 24;
          numArray1[4] = 16;
          numArray2[4] = 40;
          numArray3[4] = 32;
          numArray4[4] = 8;
          numArray1[5] = 0;
          numArray2[5] = 24;
          numArray3[5] = 16;
          numArray4[5] = 24;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 16;
          numArray4[6] = 24;
          break;
        case 1:
          numArray1[0] = 32;
          numArray2[0] = 16;
          numArray3[0] = 64;
          numArray4[0] = 64;
          numArray1[1] = 32;
          numArray2[1] = 0;
          numArray3[1] = 64;
          numArray4[1] = 16;
          numArray1[2] = 96;
          numArray2[2] = 0;
          numArray3[2] = 32;
          numArray4[2] = 48;
          numArray1[3] = 96;
          numArray2[3] = 48;
          numArray3[3] = 32;
          numArray4[3] = 48;
          numArray1[4] = 32;
          numArray2[4] = 80;
          numArray3[4] = 64;
          numArray4[4] = 16;
          numArray1[5] = 0;
          numArray2[5] = 48;
          numArray3[5] = 32;
          numArray4[5] = 48;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 32;
          numArray4[6] = 48;
          break;
      }
      float[] numArray5 = new float[10];
      float[] numArray6 = new float[10];
      float[] numArray7 = new float[10];
      float[] numArray8 = new float[10];
      let mut index1: i32 =  0;
      do
      {
        if (flag2)
        {
          numArray5[index1] = 0.0f;
          numArray6[index1] = 0.0f;
          numArray7[index1] = 0.0f;
          numArray8[index1] = 0.0f;
        }
        else
        {
          numArray5[index1] = 1f;
          numArray6[index1] = 1f;
          numArray7[index1] = 1f;
          numArray8[index1] = 0.25f;
        }
        index1 += 1;
      }
      while (index1 <= 9);
      float[] numArray9 = new float[10];
      float[] numArray10 = new float[10];
      float[] numArray11 = new float[10];
      int[] numArray12 = new int[10];
      let mut num1: i32 =  0;
      if (flag2)
      {
        numArray12[0] = 90;
        numArray12[1] = 80;
        numArray12[2] = 70;
        numArray12[3] = 60;
        numArray12[4] = 50;
        numArray12[5] = 40;
        numArray12[6] = 30;
        numArray12[7] = 20;
        numArray12[8] = 10;
        numArray12[9] = 0;
      }
      else
      {
        numArray12[0] = 60;
        numArray12[1] = 50;
        numArray12[2] = 40;
        numArray12[3] = 32;
        numArray12[4] = 25;
        numArray12[5] = 19;
        numArray12[6] = 13;
        numArray12[7] = 7;
        numArray12[8] = 0;
        numArray12[9] = 0;
      }
      let mut num2: i32 =  0;
      let mut num3: i32 =  9;
      let mut index2: i32 =  -1;
      if ( this.game.Data.RuleVar[415] > 0.0 & !flag2)
      {
        index2 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[415]));
        if (index2 == -1)
          return;
      }
      if ( this.game.Data.RuleVar[416] > 0.0 & flag2)
      {
        index2 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[416]));
        if (index2 == -1)
          return;
      }
      if (index2 > -1)
      {
        let mut length: i32 =  this.game.Data.StringListObj[index2].Length;
        for (let mut index3: i32 =  0; index3 <= length; index3 += 1)
        {
          let mut index4: i32 =  index3;
          if (flag2)
            index4 = 9 - index3;
          numArray9[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 0]) /  byte.MaxValue);
          numArray10[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 1]) /  byte.MaxValue);
          numArray11[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 2]) /  byte.MaxValue);
          numArray12[index4] = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 3]));
          numArray5[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 4]) /  byte.MaxValue);
          numArray6[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 5]) /  byte.MaxValue);
          numArray7[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 6]) /  byte.MaxValue);
          numArray8[index4] =  (Conversion.Val(this.game.Data.StringListObj[index2].Data[index3, 7]) /  byte.MaxValue);
        }
      }
      width: i32;
      height: i32;
      switch (Zoom)
      {
        case -1:
          width = 32;
          height = 24;
          break;
        case 0:
          width = 64;
          height = 48;
          break;
        default:
          width = 128;
          height = 96;
          break;
      }
      if (!(hexHeightMapData.cHeight >= num2 & hexHeightMapData.cHeight <= num3))
        return;
      let mut num4: i32 =  numArray12[hexHeightMapData.cHeight];
      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea & forInteriorSea)
        num4 = numArray12[0];
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (num4 > 0 & num4 < (int) byte.MaxValue)
      {
        ref Graphics local1 = ref useg;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
        ref local2: Bitmap = ref bitmap;
        rectangle1 = Rectangle::new(0, 0, width, height);
        let mut srcrect: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(tx, ty, width, height);
        let mut destrect: &Rectangle = &rectangle2
        double r =  numArray9[hexHeightMapData.cHeight];
        double g =  numArray10[hexHeightMapData.cHeight];
        double b =  numArray11[hexHeightMapData.cHeight];
        double a =  ( num4 /  byte.MaxValue);
        ref local3: Bitmap = ref gBitmap;
        DrawMod.DrawSimplePart2ColouredNewFast(ref local1, ref local2, srcrect, destrect,  r,  g,  b,  a, ref local3);
        num1 = num4;
      }
      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea && forInteriorSea)
        return;
      HexHeightMapData[] hexHeightMapDataArray = new HexHeightMapData[7];
      let mut index5: i32 =  1;
      do
      {
        hexHeightMapDataArray[index5] = new HexHeightMapData(ref this.game, hexHeightMapData.neighbourCoord[index5].x, hexHeightMapData.neighbourCoord[index5].y, hexHeightMapData.neighbourCoord[index5].map, forInteriorSea);
        index5 += 1;
      }
      while (index5 <= 6);
      int[] numArray13 = new int[7];
      bool flag3;
      if (!flag2 && hexHeightMapData.cHeight > 0)
      {
        let mut index6: i32 =  1;
        do
        {
          if (hexHeightMapData.neighbourCoord[index6].onmap & hexHeightMapDataArray[index6].cHeight > 0)
          {
            let mut num5: i32 =  index6 + 3;
            if (num5 > 6)
              num5 -= 6;
            let mut index7: i32 =  num5 + 1;
            if (index7 > 6)
              index7 = 1;
            let mut index8: i32 =  index7 + 1;
            if (index8 > 6)
              index8 = 1;
            let mut index9: i32 =  index6 - 1;
            if (index9 < 1)
              index9 = 6;
            let mut num6: i32 =  index9 + 3;
            let mut index10: i32 =  index6 + 1;
            if (index10 > 6)
              index10 = 1;
            if (num6 > 6)
            {
              let mut num7: i32 =  num6 - 6;
            }
            let mut index11: i32 =  index9 - 1;
            if (index11 < 1)
              index11 = 6;
            let mut index12: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index6 - 1];
            if (index12 > -1 && this.game.Data.RiverTypeObj[index12].GetRiverHeight(this.game, cx, cy, index6 - 1) < 1)
              index12 = -1;
            let mut index13: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index9 - 1];
            if (index13 > -1 && this.game.Data.RiverTypeObj[index13].GetRiverHeight(this.game, cx, cy, index9 - 1) < 1)
              index13 = -1;
            if (index12 == -1 & index13 == -1)
            {
              if (hexHeightMapDataArray[index6].riverHeightApplied[index7] == 1 && hexHeightMapData.riverHeightApplied[index6] == 0 && hexHeightMapData.riverHeightApplied[index9] == 0)
              {
                bool flag4 = true;
                if (hexHeightMapData.lineHeightLevel[1] <= hexHeightMapDataArray[index9].lineHeightLevel[1] & hexHeightMapDataArray[index9].lineHeightLevel[1] >= 1)
                  flag4 = false;
                if (hexHeightMapData.lineHeightLevel[1] <= hexHeightMapDataArray[index6].lineHeightLevel[1] & hexHeightMapDataArray[index6].lineHeightLevel[1] >= 1)
                  flag4 = false;
                if (!hexHeightMapData.neighbourCoord[index6].onmap)
                  flag4 = false;
                if (!hexHeightMapData.neighbourCoord[index9].onmap)
                  flag4 = false;
                if (!hexHeightMapData.neighbourCoord[index10].onmap)
                  flag4 = false;
                if (flag4)
                {
                  flag3 = true;
                  numArray13[index6] = 1;
                }
              }
            }
            else
            {
              let mut index14: i32 =  this.game.Data.MapObj[0].HexObj[hexHeightMapData.neighbourCoord[index6].x, hexHeightMapData.neighbourCoord[index6].y].RiverType[index7 - 1];
              if (index14 > -1 && this.game.Data.RiverTypeObj[index14].GetRiverHeight(this.game, hexHeightMapData.neighbourCoord[index6].x, hexHeightMapData.neighbourCoord[index6].y, index7 - 1) < 1)
                index14 = -1;
              if (index14 > -1)
              {
                bool flag5 = false;
                if (Math.Max(hexHeightMapDataArray[index9].cHeight, hexHeightMapDataArray[index6].cHeight) == hexHeightMapData.cHeight)
                  flag5 = true;
                if (hexHeightMapDataArray[index10].maxLines == 3)
                  flag5 = true;
                if (hexHeightMapDataArray[index11].maxLines == 3)
                  flag5 = true;
                if (Math.Abs(hexHeightMapDataArray[index9].cHeight - hexHeightMapDataArray[index6].cHeight) != 1)
                  flag5 = false;
                if (hexHeightMapData.cHeight > hexHeightMapDataArray[index6].cHeight + 1 & hexHeightMapData.cHeight <= hexHeightMapDataArray[index9].cHeight)
                  flag5 = true;
                if (index13 > -1 & hexHeightMapData.riverHeightApplied[index9] < 1 | index12 > -1 & hexHeightMapData.riverHeightApplied[index6] < 1 && hexHeightMapDataArray[index6].riverHeightApplied[index7] > 0 | hexHeightMapDataArray[index6].riverHeightApplied[index8] > 0)
                  flag5 = true;
                if (hexHeightMapDataArray[index9].cHeight == 0 | hexHeightMapDataArray[index6].cHeight == 0)
                  flag5 = false;
                if (index13 > -1 && hexHeightMapDataArray[index11].cHeight <= hexHeightMapDataArray[index9].cHeight & hexHeightMapDataArray[index11].cHeight < hexHeightMapData.cHeight)
                  flag5 = false;
                if (hexHeightMapDataArray[index6].cHeight - hexHeightMapData.cHeight == 3)
                  flag5 = false;
                if (hexHeightMapDataArray[index9].cHeight - hexHeightMapData.cHeight == 3)
                  flag5 = false;
                if (hexHeightMapData.maxLines > 0 & hexHeightMapData.lineHeightLevel[1] == 0)
                  flag5 = false;
                if (hexHeightMapData.lineHeightLevel[1] == hexHeightMapDataArray[index9].lineHeightLevel[1] & hexHeightMapDataArray[index9].lineHeightLevel[1] >= 1 && this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index9 - 1] == -1)
                  flag5 = false;
                if (hexHeightMapData.lineHeightLevel[1] == hexHeightMapDataArray[index6].lineHeightLevel[1] & hexHeightMapDataArray[index6].lineHeightLevel[1] >= 1 && this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index9 - 1] == -1)
                  flag5 = false;
                if (hexHeightMapData.lineHeightLevel[1] == hexHeightMapDataArray[index9].lineHeightLevel[2] & hexHeightMapDataArray[index9].lineHeightLevel[2] >= 1 && this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index9 - 1] == -1 & this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index6 - 1] == -1)
                  flag5 = false;
                if (hexHeightMapData.lineHeightLevel[1] == hexHeightMapDataArray[index6].lineHeightLevel[2] & hexHeightMapDataArray[index6].lineHeightLevel[2] >= 1 && this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index9 - 1] == -1 & this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index6 - 1] == -1)
                  flag5 = false;
                if (hexHeightMapData.maxLines == 3)
                  flag5 = false;
                if (!hexHeightMapData.neighbourCoord[index6].onmap)
                  flag5 = false;
                if (flag5)
                {
                  flag3 = true;
                  numArray13[index6] = 1;
                }
              }
            }
          }
          index6 += 1;
        }
        while (index6 <= 6);
      }
      if (flag1 & !flag2 & !(this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea & this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].Interior))
      {
        let mut index15: i32 =  -1;
        if ( this.game.Data.RuleVar[417] > 0.0)
          index15 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[417]));
        if (index15 > -1)
        {
          int[] numArray14 = new int[7];
          let mut num8: i32 =  0;
          bool[] flagArray = new bool[7];
          let mut num9: i32 =  0;
          let mut index16: i32 =  1;
          do
          {
            numArray14[index16] = 1;
            flagArray[index16] = false;
            if (hexHeightMapData.neighbourCoord[index16].onmap && this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType != 44)
            {
              if (hexHeightMapDataArray[index16].seaHex & hexHeightMapData.cHeight >= 0)
              {
                numArray14[index16] = 0;
                num8 += 1;
              }
              else
              {
                let mut num10: i32 =  index16 + 3;
                if (num10 > 6)
                  num10 -= 6;
                let mut index17: i32 =  num10 + 1;
                if (index17 > 6)
                  index17 = 1;
                if (hexHeightMapDataArray[index16].neighbourCoord[index17].onmap & hexHeightMapDataArray[index16].cHeight >= 0 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[hexHeightMapDataArray[index16].neighbourCoord[index17].x, hexHeightMapDataArray[index16].neighbourCoord[index17].y].LandscapeType].IsSea)
                {
                  flagArray[index16] = true;
                  num9 += 1;
                }
                if (hexHeightMapDataArray[index16].neighbourCoord[index17].onmap & hexHeightMapDataArray[index16].seaHex & hexHeightMapData.cHeight >= 0 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[hexHeightMapDataArray[index16].neighbourCoord[index17].x, hexHeightMapDataArray[index16].neighbourCoord[index17].y].LandscapeType].IsSea && this.game.Data.MapObj[0].HexObj[hexHeightMapDataArray[index16].neighbourCoord[index17].x, hexHeightMapDataArray[index16].neighbourCoord[index17].y].HeightLevel >= 0)
                {
                  flagArray[index16] = true;
                  num9 += 1;
                }
              }
            }
            index16 += 1;
          }
          while (index16 <= 6);
          if (num8 > 0 | num9 > 0)
          {
            float num11 = 0.9f;
            float num12 = 0.8f;
            float num13 = 0.7f;
            let mut num14: i32 =  175;
            if (index15 > -1)
            {
              let mut length: i32 =  this.game.Data.StringListObj[index15].Length;
              for (let mut index18: i32 =  0; index18 <= length; index18 += 1)
              {
                let mut num15: i32 =  index18;
                if (flag2)
                  num15 = 9 - index18;
                num11 =  (Conversion.Val(this.game.Data.StringListObj[index15].Data[index18, 0]) /  byte.MaxValue);
                num12 =  (Conversion.Val(this.game.Data.StringListObj[index15].Data[index18, 1]) /  byte.MaxValue);
                num13 =  (Conversion.Val(this.game.Data.StringListObj[index15].Data[index18, 2]) /  byte.MaxValue);
                num14 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[index15].Data[index18, 3]));
              }
            }
            if (num8 > 0)
            {
              let mut index19: i32 =  1;
              do
              {
                flagArray[index19] = false;
                index19 += 1;
              }
              while (index19 <= 6);
              let mut index20: i32 =  this.game.SPRITE64[numArray14[1], numArray14[2], numArray14[3], numArray14[4], numArray14[5], numArray14[6]];
              if (num14 > 0)
              {
                ref Graphics local4 = ref useg;
                bitmap: Bitmap = BitmapStore.GetBitmap(this.game.HEIGHTMAP_BEACH, Zoom);
                ref local5: Bitmap = ref bitmap;
                rectangle2 = Rectangle::new(this.game.SHEETX[index20] * width, this.game.SHEETY[index20] * height, width, height);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, width, height);
                let mut destrect: &Rectangle = &rectangle1
                double r =  num11;
                double g =  num12;
                double b =  num13;
                double a =  ( num14 /  byte.MaxValue);
                DrawMod.DrawSimplePart2ColouredNew(ref local4, ref local5, srcrect, destrect,  r,  g,  b,  a);
              }
            }
            let mut index21: i32 =  1;
            do
            {
              if (flagArray[index21])
              {
                let mut num16: i32 =  index21 - 1;
              }
              index21 += 1;
            }
            while (index21 <= 6);
          }
        }
      }
      bitmap1: Bitmap;
      bitmap2: Bitmap;
      for (let mut maxLines1: i32 =  hexHeightMapData.maxLines; maxLines1 >= 1; maxLines1 += -1)
      {
        if (hexHeightMapData.lineHeightLevel[maxLines1] > -999)
        {
          int[] numArray15 = new int[7];
          bool flag6 = false;
          int[] numArray16 = new int[7];
          int[] numArray17 = new int[7];
          int[] numArray18 = new int[7];
          bool[] flagArray = new bool[7];
          let mut index22: i32 =  1;
          do
          {
            if (hexHeightMapData.neighbourHeight[index22] <= hexHeightMapData.lineHeightLevel[maxLines1])
            {
              numArray15[index22] = 0;
              flag6 = true;
              numArray16[index22] = 0;
            }
            else
            {
              numArray15[index22] = 1;
              numArray16[index22] = 1;
            }
            index22 += 1;
          }
          while (index22 <= 6);
          let mut index23: i32 =  1;
          do
          {
            if (numArray15[index23] == 1)
            {
              let mut index24: i32 =  index23 - 1;
              let mut index25: i32 =  index23 + 1;
              if (index24 < 1)
                index24 = 6;
              if (index25 > 6)
                index25 = 1;
              if (numArray15[index24] == 0 & numArray15[index25] == 0 && hexHeightMapDataArray[index23].maxLines > 0 && hexHeightMapDataArray[index23].lineHeightLevel[maxLines1] != hexHeightMapData.lineHeightLevel[maxLines1])
              {
                numArray15[index23] = 0;
                numArray16[index23] = 0;
                flagArray[index23] = true;
              }
              if (((numArray15[index23] == 1 ? 1 : 0) & 1) != 0)
              {
                let mut num17: i32 =  index23 + 3;
                if (num17 > 6)
                  num17 -= 6;
                let mut index26: i32 =  num17 - 1;
                if (index26 < 1)
                  index26 = 6;
                let mut index27: i32 =  num17 + 1;
                if (index27 > 6)
                  index27 = 1;
                if ((numArray15[index25] == 0 | hexHeightMapDataArray[index23].HexRiverHeight[index26] > 0) & (numArray15[index24] == 0 | hexHeightMapDataArray[index23].HexRiverHeight[index27] > 0) && hexHeightMapDataArray[index23].maxLines > 0 & (hexHeightMapDataArray[index23].HexRiverHeight[index26] > 0 | hexHeightMapDataArray[index23].HexRiverHeight[index27] > 0) && hexHeightMapDataArray[index23].cHeight - 2 < hexHeightMapData.lineHeightLevel[maxLines1] && !(maxLines1 == 1 & hexHeightMapDataArray[index23].lineHeightLevel[maxLines1] == hexHeightMapData.lineHeightLevel[maxLines1]))
                {
                  if (hexHeightMapData.lineHeightLevel[maxLines1] == hexHeightMapDataArray[index25].lineHeightLevel[maxLines1])
                  {
                    numArray15[index23] = 0;
                    flagArray[index23] = true;
                    numArray16[index23] = 0;
                  }
                  if (hexHeightMapData.lineHeightLevel[maxLines1] == hexHeightMapDataArray[index24].lineHeightLevel[maxLines1])
                  {
                    numArray15[index23] = 0;
                    flagArray[index23] = true;
                    numArray16[index23] = 0;
                  }
                }
              }
            }
            index23 += 1;
          }
          while (index23 <= 6);
          let mut index28: i32 =  1;
          do
          {
            numArray18[index28] = 0;
            if (numArray16[index28] == 1)
            {
              let mut index29: i32 =  index28 - 1;
              let mut index30: i32 =  index28 + 1;
              if (index29 < 1)
                index29 = 6;
              if (index30 > 6)
                index30 = 1;
              if (flag2)
              {
                let mut num18: i32 =  hexHeightMapDataArray[index29].cHeight;
                let mut num19: i32 =  hexHeightMapDataArray[index30].cHeight;
                if (!hexHeightMapDataArray[index29].seaHex)
                  num18 = 9;
                if (!hexHeightMapDataArray[index30].seaHex)
                  num19 = 9;
                if (num18 > num19)
                  numArray18[index28] = 1;
                if (num18 < num19)
                  numArray18[index28] = 2;
              }
              else
              {
                if (hexHeightMapDataArray[index29].cHeight > hexHeightMapDataArray[index30].cHeight)
                  numArray18[index28] = 1;
                if (hexHeightMapDataArray[index29].cHeight < hexHeightMapDataArray[index30].cHeight)
                  numArray18[index28] = 2;
              }
              if (!flag2)
              {
                let mut index31: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index29 - 1];
                if (index31 > -1 && this.game.Data.RiverTypeObj[index31].GetRiverHeight(this.game, cx, cy, index29 - 1) < 1)
                  index31 = -1;
                let mut index32: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index30 - 1];
                if (index32 > -1 && this.game.Data.RiverTypeObj[index32].GetRiverHeight(this.game, cx, cy, index30 - 1) < 1)
                  index32 = -1;
                if (index31 > -1)
                {
                  let mut num20: i32 =  1;
                  if (hexHeightMapData.lineHeightLevel[maxLines1] == hexHeightMapData.cHeight - num20)
                    numArray18[index28] = 2;
                }
                if (index32 > -1)
                {
                  let mut num21: i32 =  1;
                  if (hexHeightMapData.lineHeightLevel[maxLines1] == hexHeightMapData.cHeight - num21)
                    numArray18[index28] = 1;
                }
              }
              if (numArray15[index29] > 0 & numArray15[index30] > 0)
                numArray16[index28] = 0;
              if (!hexHeightMapData.neighbourCoord[index28].onmap)
                numArray16[index28] = 0;
            }
            index28 += 1;
          }
          while (index28 <= 6);
          bool flag7 = false;
          let mut index33: i32 =  1;
          do
          {
            if (numArray16[index33] == 1)
            {
              let mut num22: i32 =  999;
              let mut maxLines2: i32 =  hexHeightMapDataArray[index33].maxLines;
              for (let mut index34: i32 =  1; index34 <= maxLines2; index34 += 1)
              {
                if (hexHeightMapDataArray[index33].lineHeightLevel[index34] == hexHeightMapData.lineHeightLevel[maxLines1])
                {
                  num22 = index34;
                  break;
                }
              }
              if (num22 < 999 & num22 != maxLines1)
              {
                numArray16[index33] = 3;
                numArray17[index33] = num22;
                flag7 = true;
              }
              else
                numArray16[index33] = 2;
            }
            index33 += 1;
          }
          while (index33 <= 6);
          if (flag6)
          {
            if (!flag7)
            {
              let mut index35: i32 =  this.game.SPRITE64[numArray15[1], numArray15[2], numArray15[3], numArray15[4], numArray15[5], numArray15[6]];
              let mut num23: i32 =  numArray12[hexHeightMapData.lineHeightLevel[maxLines1]];
              let mut index36: i32 =  hexHeightMapData.lineHeightLevel[maxLines1];
              if (num23 > 0 & num23 < (int) byte.MaxValue)
              {
                let mut num24: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num23 - num1)) /  ((int) byte.MaxValue - num1));
                if (maxLines1 == 1)
                {
                  ref Graphics local6 = ref useg;
                  bitmap1 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW1, Zoom);
                  ref local7: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray9[index36];
                  double g =  numArray10[index36];
                  double b =  numArray11[index36];
                  double a =  ( num24 /  byte.MaxValue);
                  bitmap2 = (Bitmap) null;
                  ref local8: Bitmap = ref bitmap2;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local6, ref local7, srcrect, destrect,  r,  g,  b,  a, ref local8);
                }
                if (maxLines1 == 2)
                {
                  ref Graphics local9 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW2, Zoom);
                  ref local10: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray9[index36];
                  double g =  numArray10[index36];
                  double b =  numArray11[index36];
                  double a =  ( num24 /  byte.MaxValue);
                  bitmap1 = (Bitmap) null;
                  ref local11: Bitmap = ref bitmap1;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local9, ref local10, srcrect, destrect,  r,  g,  b,  a, ref local11);
                }
                if (maxLines1 == 3)
                {
                  ref Graphics local12 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW3, Zoom);
                  ref local13: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray9[index36];
                  double g =  numArray10[index36];
                  double b =  numArray11[index36];
                  double a =  ( num24 /  byte.MaxValue);
                  bitmap1 = (Bitmap) null;
                  ref local14: Bitmap = ref bitmap1;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local12, ref local13, srcrect, destrect,  r,  g,  b,  a, ref local14);
                }
                num1 = num23;
              }
              if ( numArray8[index36] == 1.0)
              {
                if (maxLines1 == 1)
                {
                  ref Graphics local15 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
                  ref local16: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
                }
                if (maxLines1 == 2)
                {
                  ref Graphics local17 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
                  ref local18: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect, destrect);
                }
                if (maxLines1 == 3)
                {
                  ref Graphics local19 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
                  ref local20: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local19, ref local20, srcrect, destrect);
                }
              }
              else
              {
                if (maxLines1 == 1)
                {
                  ref Graphics local21 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
                  ref local22: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray5[index36];
                  double g =  numArray6[index36];
                  double b =  numArray7[index36];
                  double a =  numArray8[index36];
                  bitmap1 = (Bitmap) null;
                  ref local23: Bitmap = ref bitmap1;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local21, ref local22, srcrect, destrect,  r,  g,  b,  a, ref local23);
                }
                if (maxLines1 == 2)
                {
                  ref Graphics local24 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
                  ref local25: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray5[index36];
                  double g =  numArray6[index36];
                  double b =  numArray7[index36];
                  double a =  numArray8[index36];
                  bitmap1 = (Bitmap) null;
                  ref local26: Bitmap = ref bitmap1;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local24, ref local25, srcrect, destrect,  r,  g,  b,  a, ref local26);
                }
                if (maxLines1 == 3)
                {
                  ref Graphics local27 = ref useg;
                  bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
                  ref local28: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index35] * width, this.game.SHEETY[index35] * height, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  numArray5[index36];
                  double g =  numArray6[index36];
                  double b =  numArray7[index36];
                  double a =  numArray8[index36];
                  bitmap1 = (Bitmap) null;
                  ref local29: Bitmap = ref bitmap1;
                  DrawMod.DrawSimplePart2ColouredNewFast(ref local27, ref local28, srcrect, destrect,  r,  g,  b,  a, ref local29);
                }
              }
            }
            else if (flag7)
            {
              let mut num25: i32 =  num1;
              let mut index37: i32 =  0;
              do
              {
                num26: i32;
                num27: i32;
                num28: i32;
                if (index37 >= 1 & index37 <= 3 & numArray16[index37] == 3 & maxLines1 > 1)
                {
                  num26 = 0;
                  num27 = 0;
                  num28 = 0;
                  let mut num29: i32 =  (index37 - 1) * 2;
                  let mut num30: i32 =  0;
                  if (numArray18[index37] == 1)
                    num29 += 1;
                  if (numArray17[index37] == 1 & maxLines1 == 2)
                    num30 = 0;
                  if (numArray17[index37] == 3 & maxLines1 == 2)
                    num30 = 1;
                  if (numArray17[index37] == 1 & maxLines1 == 3)
                    num30 = 2;
                  if (numArray17[index37] == 2 & maxLines1 == 3)
                    num30 = 3;
                  let mut num31: i32 =  numArray12[hexHeightMapData.lineHeightLevel[maxLines1]];
                  let mut index38: i32 =  hexHeightMapData.lineHeightLevel[maxLines1];
                  if (num31 > 0 & num31 < (int) byte.MaxValue)
                  {
                    let mut num32: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num31 - num1)) /  ((int) byte.MaxValue - num1));
                    ref Graphics local30 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_SHADOW, Zoom);
                    ref local31: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num30 * width + numArray1[index37], num29 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    double r =  numArray9[index38];
                    double g =  numArray10[index38];
                    double b =  numArray11[index38];
                    double a =  ( num32 /  byte.MaxValue);
                    bitmap1 = (Bitmap) null;
                    ref local32: Bitmap = ref bitmap1;
                    DrawMod.DrawSimplePart2ColouredNewFast(ref local30, ref local31, srcrect, destrect,  r,  g,  b,  a, ref local32);
                    if (num31 > num25)
                      num25 = num31;
                  }
                  if ( numArray8[index38] == 1.0)
                  {
                    ref Graphics local33 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                    ref local34: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num30 * width + numArray1[index37], num29 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local33, ref local34, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local35 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                    ref local36: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num30 * width + numArray1[index37], num29 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    double r =  numArray5[index38];
                    double g =  numArray6[index38];
                    double b =  numArray7[index38];
                    double a =  numArray8[index38];
                    bitmap1 = (Bitmap) null;
                    ref local37: Bitmap = ref bitmap1;
                    DrawMod.DrawSimplePart2ColouredNewFast(ref local35, ref local36, srcrect, destrect,  r,  g,  b,  a, ref local37);
                  }
                }
                else if (index37 >= 4 & index37 <= 6 & numArray16[index37] == 3 & numArray17[index37] == 1)
                {
                  num26 = 0;
                  num27 = 0;
                  num28 = 0;
                  let mut num33: i32 =  (index37 - 1) * 2;
                  let mut num34: i32 =  0;
                  if (index37 == 5)
                    num33 += 2;
                  if (index37 == 6)
                    num33 -= 2;
                  if (numArray18[index37] == 2)
                    num33 += 1;
                  if (index37 == 4)
                  {
                    if (numArray17[index37] == 1 & maxLines1 == 2)
                      num34 = 0;
                    if (numArray17[index37] == 3 & maxLines1 == 2)
                      num34 = 1;
                    if (numArray17[index37] == 1 & maxLines1 == 3)
                      num34 = 2;
                    if (numArray17[index37] == 2 & maxLines1 == 3)
                      num34 = 3;
                  }
                  else if (index37 == 5 | index37 == 6)
                  {
                    if (numArray17[index37] == 1 & maxLines1 == 2)
                      num34 = 3;
                    if (numArray17[index37] == 3 & maxLines1 == 2)
                      num34 = 2;
                    if (numArray17[index37] == 1 & maxLines1 == 3)
                      num34 = 1;
                    if (numArray17[index37] == 2 & maxLines1 == 3)
                      num34 = 0;
                  }
                  let mut num35: i32 =  numArray12[hexHeightMapData.lineHeightLevel[maxLines1]];
                  let mut index39: i32 =  hexHeightMapData.lineHeightLevel[maxLines1];
                  if (num35 > 0 & num35 < (int) byte.MaxValue)
                  {
                    let mut num36: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num35 - num1)) /  ((int) byte.MaxValue - num1));
                    ref Graphics local38 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_SHADOW, Zoom);
                    ref local39: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num34 * width + numArray1[index37], num33 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    double r =  numArray9[index39];
                    double g =  numArray10[index39];
                    double b =  numArray11[index39];
                    double a =  ( num36 /  byte.MaxValue);
                    bitmap1 = (Bitmap) null;
                    ref local40: Bitmap = ref bitmap1;
                    DrawMod.DrawSimplePart2ColouredNewFast(ref local38, ref local39, srcrect, destrect,  r,  g,  b,  a, ref local40);
                    if (num35 > num25)
                      num25 = num35;
                  }
                  if ( numArray8[index39] == 1.0)
                  {
                    ref Graphics local41 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                    ref local42: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num34 * width + numArray1[index37], num33 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local41, ref local42, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local43 = ref useg;
                    bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                    ref local44: Bitmap = ref bitmap2;
                    rectangle2 = Rectangle::new(num34 * width + numArray1[index37], num33 * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                    let mut destrect: &Rectangle = &rectangle1
                    double r =  numArray5[index39];
                    double g =  numArray6[index39];
                    double b =  numArray7[index39];
                    double a =  numArray8[index39];
                    bitmap1 = (Bitmap) null;
                    ref local45: Bitmap = ref bitmap1;
                    DrawMod.DrawSimplePart2ColouredNewFast(ref local43, ref local44, srcrect, destrect,  r,  g,  b,  a, ref local45);
                  }
                }
                else
                {
                  let mut index40: i32 =  this.game.SPRITE64[numArray15[1], numArray15[2], numArray15[3], numArray15[4], numArray15[5], numArray15[6]];
                  let mut num37: i32 =  numArray12[hexHeightMapData.lineHeightLevel[maxLines1]];
                  let mut index41: i32 =  hexHeightMapData.lineHeightLevel[maxLines1];
                  if (num37 > 0 & num37 < (int) byte.MaxValue)
                  {
                    let mut num38: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num37 - num1)) /  ((int) byte.MaxValue - num1));
                    if (maxLines1 == 1)
                    {
                      ref Graphics local46 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW1, Zoom);
                      ref local47: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray9[index41];
                      double g =  numArray10[index41];
                      double b =  numArray11[index41];
                      double a =  ( num38 /  byte.MaxValue);
                      bitmap1 = (Bitmap) null;
                      ref local48: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local46, ref local47, srcrect, destrect,  r,  g,  b,  a, ref local48);
                    }
                    if (maxLines1 == 2)
                    {
                      ref Graphics local49 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW2, Zoom);
                      ref local50: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray9[index41];
                      double g =  numArray10[index41];
                      double b =  numArray11[index41];
                      double a =  ( num38 /  byte.MaxValue);
                      bitmap1 = (Bitmap) null;
                      ref local51: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local49, ref local50, srcrect, destrect,  r,  g,  b,  a, ref local51);
                    }
                    if (maxLines1 == 3)
                    {
                      ref Graphics local52 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW3, Zoom);
                      ref local53: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray9[index41];
                      double g =  numArray10[index41];
                      double b =  numArray11[index41];
                      double a =  ( num38 /  byte.MaxValue);
                      bitmap1 = (Bitmap) null;
                      ref local54: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local52, ref local53, srcrect, destrect,  r,  g,  b,  a, ref local54);
                    }
                    if (num37 > num25)
                      num25 = num37;
                  }
                  if ( numArray8[index41] == 1.0)
                  {
                    if (maxLines1 == 1)
                    {
                      ref Graphics local55 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
                      ref local56: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local55, ref local56, srcrect, destrect);
                    }
                    if (maxLines1 == 2)
                    {
                      ref Graphics local57 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
                      ref local58: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local57, ref local58, srcrect, destrect);
                    }
                    if (maxLines1 == 3)
                    {
                      ref Graphics local59 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
                      ref local60: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local59, ref local60, srcrect, destrect);
                    }
                  }
                  else
                  {
                    if (maxLines1 == 1)
                    {
                      ref Graphics local61 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
                      ref local62: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray5[index41];
                      double g =  numArray6[index41];
                      double b =  numArray7[index41];
                      double a =  numArray8[index41];
                      bitmap1 = (Bitmap) null;
                      ref local63: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local61, ref local62, srcrect, destrect,  r,  g,  b,  a, ref local63);
                    }
                    if (maxLines1 == 2)
                    {
                      ref Graphics local64 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
                      ref local65: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray5[index41];
                      double g =  numArray6[index41];
                      double b =  numArray7[index41];
                      double a =  numArray8[index41];
                      bitmap1 = (Bitmap) null;
                      ref local66: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local64, ref local65, srcrect, destrect,  r,  g,  b,  a, ref local66);
                    }
                    if (maxLines1 == 3)
                    {
                      ref Graphics local67 = ref useg;
                      bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
                      ref local68: Bitmap = ref bitmap2;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index40] * width + numArray1[index37], this.game.SHEETY[index40] * height + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + numArray1[index37], ty + numArray2[index37], numArray3[index37], numArray4[index37]);
                      let mut destrect: &Rectangle = &rectangle1
                      double r =  numArray5[index41];
                      double g =  numArray6[index41];
                      double b =  numArray7[index41];
                      double a =  numArray8[index41];
                      bitmap1 = (Bitmap) null;
                      ref local69: Bitmap = ref bitmap1;
                      DrawMod.DrawSimplePart2ColouredNewFast(ref local67, ref local68, srcrect, destrect,  r,  g,  b,  a, ref local69);
                    }
                  }
                }
                index37 += 1;
              }
              while (index37 <= 6);
              num1 = num25;
            }
          }
        }
      }
      if (!(flag3 & !flag2))
        return;
      let mut index42: i32 =  1;
      do
      {
        if (numArray13[index42] == 1 & hexHeightMapData.cHeight > 0)
        {
          let mut num39: i32 =  4;
          let mut num40: i32 =  index42 - 1;
          let mut index43: i32 =  index42 - 1;
          if (index43 < 1)
            index43 = 6;
          if (hexHeightMapData.cHeight == hexHeightMapDataArray[index42].cHeight & hexHeightMapData.cHeight == hexHeightMapDataArray[index43].cHeight & hexHeightMapDataArray[index42].maxLines > 1 && hexHeightMapDataArray[index42].lineHeightLevel[2] == hexHeightMapData.cHeight - 1)
            num39 = 5;
          let mut num41: i32 =  numArray12[hexHeightMapData.cHeight - 1];
          let mut index44: i32 =  hexHeightMapData.cHeight - 1;
          if (num41 > 0 & num41 < (int) byte.MaxValue)
          {
            let mut num42: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num41 - num1)) /  ((int) byte.MaxValue - num1));
            if (num42 > 0)
            {
              ref Graphics local70 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_SHADOW, Zoom);
              ref local71: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(num39 * width, num40 * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              double r =  numArray9[index44];
              double g =  numArray10[index44];
              double b =  numArray11[index44];
              double a =  ( num42 /  byte.MaxValue);
              bitmap1 = (Bitmap) null;
              ref local72: Bitmap = ref bitmap1;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local70, ref local71, srcrect, destrect,  r,  g,  b,  a, ref local72);
              num1 = num41;
            }
          }
          if ( numArray8[index44] == 1.0)
          {
            ref Graphics local73 = ref useg;
            bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
            ref local74: Bitmap = ref bitmap2;
            rectangle2 = Rectangle::new(num39 * width, num40 * height, width, height);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, width, height);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local73, ref local74, srcrect, destrect);
          }
          else if ( numArray8[index44] > 0.0)
          {
            ref Graphics local75 = ref useg;
            bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
            ref local76: Bitmap = ref bitmap2;
            rectangle2 = Rectangle::new(num39 * width, num40 * height, width, height);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, width, height);
            let mut destrect: &Rectangle = &rectangle1
            double r =  numArray5[index44];
            double g =  numArray6[index44];
            double b =  numArray7[index44];
            double a =  numArray8[index44];
            bitmap1 = (Bitmap) null;
            ref local77: Bitmap = ref bitmap1;
            DrawMod.DrawSimplePart2ColouredNewFast(ref local75, ref local76, srcrect, destrect,  r,  g,  b,  a, ref local77);
          }
        }
        index42 += 1;
      }
      while (index42 <= 6);
    }

    pub void DrawCanyon(
      ref Graphics useg,
      cx: i32,
      cy: i32,
      cmap: i32,
      tx: i32,
      ty: i32,
      Zoom: i32,
      ref gBitmap: Bitmap = null)
    {
      if (!this.game.AllowHeightMap)
        return;
      int[] numArray1 = new int[7];
      int[] numArray2 = new int[7];
      int[] numArray3 = new int[7];
      int[] numArray4 = new int[7];
      switch (Zoom)
      {
        case -1:
          numArray1[0] = 8;
          numArray2[0] = 4;
          numArray3[0] = 16;
          numArray4[0] = 16;
          numArray1[1] = 8;
          numArray2[1] = 0;
          numArray3[1] = 16;
          numArray4[1] = 4;
          numArray1[2] = 24;
          numArray2[2] = 0;
          numArray3[2] = 8;
          numArray4[2] = 12;
          numArray1[3] = 24;
          numArray2[3] = 12;
          numArray3[3] = 8;
          numArray4[3] = 12;
          numArray1[4] = 8;
          numArray2[4] = 20;
          numArray3[4] = 16;
          numArray4[4] = 4;
          numArray1[5] = 0;
          numArray2[5] = 12;
          numArray3[5] = 8;
          numArray4[5] = 12;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 8;
          numArray4[6] = 12;
          break;
        case 0:
          numArray1[0] = 16;
          numArray2[0] = 8;
          numArray3[0] = 32;
          numArray4[0] = 32;
          numArray1[1] = 16;
          numArray2[1] = 0;
          numArray3[1] = 32;
          numArray4[1] = 8;
          numArray1[2] = 48;
          numArray2[2] = 0;
          numArray3[2] = 16;
          numArray4[2] = 24;
          numArray1[3] = 48;
          numArray2[3] = 24;
          numArray3[3] = 16;
          numArray4[3] = 24;
          numArray1[4] = 16;
          numArray2[4] = 40;
          numArray3[4] = 32;
          numArray4[4] = 8;
          numArray1[5] = 0;
          numArray2[5] = 24;
          numArray3[5] = 16;
          numArray4[5] = 24;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 16;
          numArray4[6] = 24;
          break;
        case 1:
          numArray1[0] = 32;
          numArray2[0] = 16;
          numArray3[0] = 64;
          numArray4[0] = 64;
          numArray1[1] = 32;
          numArray2[1] = 0;
          numArray3[1] = 64;
          numArray4[1] = 16;
          numArray1[2] = 96;
          numArray2[2] = 0;
          numArray3[2] = 32;
          numArray4[2] = 48;
          numArray1[3] = 96;
          numArray2[3] = 48;
          numArray3[3] = 32;
          numArray4[3] = 48;
          numArray1[4] = 32;
          numArray2[4] = 80;
          numArray3[4] = 64;
          numArray4[4] = 16;
          numArray1[5] = 0;
          numArray2[5] = 48;
          numArray3[5] = 32;
          numArray4[5] = 48;
          numArray1[6] = 0;
          numArray2[6] = 0;
          numArray3[6] = 32;
          numArray4[6] = 48;
          break;
      }
      float[] numArray5 = new float[10];
      float[] numArray6 = new float[10];
      float[] numArray7 = new float[10];
      float[] numArray8 = new float[10];
      let mut index1: i32 =  0;
      do
      {
        numArray5[index1] = 1f;
        numArray6[index1] = 1f;
        numArray7[index1] = 1f;
        numArray8[index1] = 0.125f;
        if (index1 == 0)
          numArray8[index1] = 0.25f;
        index1 += 1;
      }
      while (index1 <= 9);
      float[] numArray9 = new float[10];
      float[] numArray10 = new float[10];
      float[] numArray11 = new float[10];
      int[] numArray12 = new int[10];
      let mut num1: i32 =  0;
      int[] numArray13 = new int[7];
      numArray12[0] = 100;
      numArray12[1] = 40;
      numArray12[2] = 0;
      width: i32;
      height: i32;
      switch (Zoom)
      {
        case -1:
          width = 32;
          height = 24;
          break;
        case 0:
          width = 64;
          height = 48;
          break;
        default:
          width = 128;
          height = 96;
          break;
      }
      Coordinate[] coordinateArray = new Coordinate[7];
      let mut tfacing: i32 =  1;
      do
      {
        coordinateArray[tfacing] = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
        numArray13[tfacing] = 0;
        tfacing += 1;
      }
      while (tfacing <= 6);
      let mut num2: i32 =  2;
      do
      {
        int[] numArray14 = new int[7];
        bool flag1 = false;
        int[] numArray15 = new int[7];
        bool flag2 = false;
        let mut index2: i32 =  1;
        do
        {
          if (coordinateArray[index2].onmap)
          {
            if (this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index2 - 1] == 4)
            {
              numArray14[index2] = 0;
              flag1 = true;
            }
            else
              numArray14[index2] = 1;
            let mut num3: i32 =  index2 + 4;
            if (num3 > 6)
              num3 -= 6;
            if (this.game.Data.MapObj[0].HexObj[coordinateArray[index2].x, coordinateArray[index2].y].RiverType[num3 - 1] == 4)
            {
              let mut num4: i32 =  index2 - 1;
              if (num4 < 1)
                num4 = 6;
              if (this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[num4 - 1] != 4 && this.game.Data.MapObj[0].HexObj[cx, cy].RiverType[index2 - 1] != 4)
              {
                numArray15[index2] = 1;
                flag2 = true;
              }
            }
          }
          index2 += 1;
        }
        while (index2 <= 6);
        bitmap1: Bitmap;
        Rectangle rectangle1;
        Rectangle rectangle2;
        bitmap2: Bitmap;
        if (flag1)
        {
          let mut index3: i32 =  this.game.SPRITE64[numArray14[1], numArray14[2], numArray14[3], numArray14[4], numArray14[5], numArray14[6]];
          let mut num5: i32 =  numArray12[num2 - 1];
          let mut index4: i32 =  num2 - 1;
          if (num5 > 0 & num5 < (int) byte.MaxValue)
          {
            let mut num6: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num5 - num1)) /  ((int) byte.MaxValue - num1));
            if (num2 == 1)
            {
              ref Graphics local1 = ref useg;
              bitmap1 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW1, Zoom);
              ref local2: Bitmap = ref bitmap1;
              rectangle1 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle1
              rectangle2 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle2
              double r =  numArray9[index4];
              double g =  numArray10[index4];
              double b =  numArray11[index4];
              double a =  ( num6 /  byte.MaxValue);
              bitmap2 = (Bitmap) null;
              ref local3: Bitmap = ref bitmap2;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local1, ref local2, srcrect, destrect,  r,  g,  b,  a, ref local3);
            }
            if (num2 == 2)
            {
              ref Graphics local4 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_SHADOW2, Zoom);
              ref local5: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              double r =  numArray9[index4];
              double g =  numArray10[index4];
              double b =  numArray11[index4];
              double a =  ( num6 /  byte.MaxValue);
              bitmap1 = (Bitmap) null;
              ref local6: Bitmap = ref bitmap1;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local4, ref local5, srcrect, destrect,  r,  g,  b,  a, ref local6);
            }
            num1 = num5;
          }
          if ( numArray8[index4] == 1.0)
          {
            if (num2 == 1)
            {
              ref Graphics local7 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
              ref local8: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect, destrect);
            }
            if (num2 == 2)
            {
              ref Graphics local9 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
              ref local10: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
            }
            if (num2 == 3)
            {
              ref Graphics local11 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
              ref local12: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect, destrect);
            }
          }
          else
          {
            if (num2 == 1)
            {
              ref Graphics local13 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE1, Zoom);
              ref local14: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              double r =  numArray5[index4];
              double g =  numArray6[index4];
              double b =  numArray7[index4];
              double a =  numArray8[index4];
              bitmap1 = (Bitmap) null;
              ref local15: Bitmap = ref bitmap1;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local13, ref local14, srcrect, destrect,  r,  g,  b,  a, ref local15);
            }
            if (num2 == 2)
            {
              ref Graphics local16 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE2, Zoom);
              ref local17: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              double r =  numArray5[index4];
              double g =  numArray6[index4];
              double b =  numArray7[index4];
              double a =  numArray8[index4];
              bitmap1 = (Bitmap) null;
              ref local18: Bitmap = ref bitmap1;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local16, ref local17, srcrect, destrect,  r,  g,  b,  a, ref local18);
            }
            if (num2 == 3)
            {
              ref Graphics local19 = ref useg;
              bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_LINE3, Zoom);
              ref local20: Bitmap = ref bitmap2;
              rectangle2 = Rectangle::new(this.game.SHEETX[index3] * width, this.game.SHEETY[index3] * height, width, height);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, width, height);
              let mut destrect: &Rectangle = &rectangle1
              double r =  numArray5[index4];
              double g =  numArray6[index4];
              double b =  numArray7[index4];
              double a =  numArray8[index4];
              bitmap1 = (Bitmap) null;
              ref local21: Bitmap = ref bitmap1;
              DrawMod.DrawSimplePart2ColouredNewFast(ref local19, ref local20, srcrect, destrect,  r,  g,  b,  a, ref local21);
            }
          }
        }
        if (flag2)
        {
          let mut index5: i32 =  1;
          do
          {
            if (numArray15[index5] == 1)
            {
              let mut num7: i32 =  4;
              if (num2 == 2)
                num7 = 5;
              let mut num8: i32 =  index5 - 1;
              let mut num9: i32 =  index5 - 1;
              let mut num10: i32 =  numArray12[num2 - 1];
              let mut index6: i32 =  num2 - 1;
              if (num10 > 0 & num10 < (int) byte.MaxValue)
              {
                let mut num11: i32 =  (int) Math.Round( ((int) byte.MaxValue * (num10 - numArray13[index5])) /  ((int) byte.MaxValue - numArray13[index5]));
                ref Graphics local22 = ref useg;
                bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_SHADOW, Zoom);
                ref local23: Bitmap = ref bitmap2;
                rectangle2 = Rectangle::new(num7 * width, num8 * height, width, height);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, width, height);
                let mut destrect: &Rectangle = &rectangle1
                double r =  numArray9[index6];
                double g =  numArray10[index6];
                double b =  numArray11[index6];
                double a =  ( num11 /  byte.MaxValue);
                bitmap1 = (Bitmap) null;
                ref local24: Bitmap = ref bitmap1;
                DrawMod.DrawSimplePart2ColouredNewFast(ref local22, ref local23, srcrect, destrect,  r,  g,  b,  a, ref local24);
                numArray13[index5] = num10;
              }
              if ( numArray8[index6] == 1.0)
              {
                ref Graphics local25 = ref useg;
                bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                ref local26: Bitmap = ref bitmap2;
                rectangle2 = Rectangle::new(num7 * width, num8 * height, width, height);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, width, height);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local25, ref local26, srcrect, destrect);
              }
              else
              {
                ref Graphics local27 = ref useg;
                bitmap2 = BitmapStore.GetBitmap(this.game.HEIGHTMAP_TRANS_LINE, Zoom);
                ref local28: Bitmap = ref bitmap2;
                rectangle2 = Rectangle::new(num7 * width, num8 * height, width, height);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, width, height);
                let mut destrect: &Rectangle = &rectangle1
                double r =  numArray5[index6];
                double g =  numArray6[index6];
                double b =  numArray7[index6];
                double a =  numArray8[index6];
                bitmap1 = (Bitmap) null;
                ref local29: Bitmap = ref bitmap1;
                DrawMod.DrawSimplePart2ColouredNewFast(ref local27, ref local28, srcrect, destrect,  r,  g,  b,  a, ref local29);
              }
            }
            index5 += 1;
          }
          while (index5 <= 6);
        }
        num2 += -1;
      }
      while (num2 >= 1);
    }

    pub SimpleStringList DrawStandardShadowEmpireFrame(
      Graphics g,
      temp1: i32,
      temp2: i32,
      bool isGameLoop,
      bool isVidCom)
    {
      SimpleStringList simpleStringList = SimpleStringList::new();
      let mut tx1: i32 =  312;
      let mut screenWidth: i32 =  this.game.ScreenWidth;
      let mut screenHeight: i32 =  this.game.ScreenHeight;
      let mut tdata5_1: i32 =  101;
      bool grayedOut;
      bool active;
      if (isGameLoop)
      {
        grayedOut = true;
        active = false;
      }
      if (isVidCom)
      {
        grayedOut = false;
        active = false;
      }
      if (this.game.EditObj.TestEarlyCinematics == 1 & isVidCom)
      {
        grayedOut = true;
        active = false;
        tdata5_1 = -1;
      }
      Rectangle rectangle1 = this.DrawOneTab(g, active, tx1, "MAP", -1, grayedOut: grayedOut) with
      {
        Y = 36
      };
      rectangle1.Height -= 36;
      if (isVidCom)
        simpleStringList.Add("Click to go to MAP mode. [Shortkey Escape]", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, tdata5_1, false);
      if (isGameLoop)
        simpleStringList.Add("Inaccesible during the AIs turn.", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, -1, false);
      let mut tx2: i32 =  tx1 + 68;
      let mut tdata5_2: i32 =  102;
      if (isGameLoop)
      {
        grayedOut = true;
        active = false;
      }
      if (isVidCom)
      {
        grayedOut = false;
        active = false;
      }
      if (this.game.EditObj.TestEarlyCinematics == 1 & isVidCom)
      {
        grayedOut = true;
        active = false;
        tdata5_2 = -1;
      }
      rectangle1 = this.DrawOneTab(g, active, tx2, "HIS", -1, grayedOut: grayedOut) with
      {
        Y = 36
      };
      rectangle1.Height -= 36;
      if (isVidCom)
        simpleStringList.Add("Click to go to HISTORY mode. [Shortkey H]", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, tdata5_2, false);
      if (isGameLoop)
        simpleStringList.Add("Inaccesible during the AIs turn.", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, -1, false);
      let mut tx3: i32 =  tx2 + 68;
      if (isGameLoop)
      {
        grayedOut = true;
        active = false;
      }
      if (isVidCom)
      {
        grayedOut = false;
        active = true;
      }
      if (this.game.EditObj.TestEarlyCinematics == 1 & isVidCom)
      {
        grayedOut = false;
        active = true;
      }
      rectangle1 = this.DrawOneTab(g, active, tx3, "VID", -1, grayedOut: grayedOut) with
      {
        Y = 36
      };
      rectangle1.Height -= 36;
      if (isVidCom)
        simpleStringList.Add("You are currently in VIDCOM mode.", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, -1, false);
      if (isGameLoop)
        simpleStringList.Add("Inaccesible during the AIs turn.", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, -1, false);
      let mut tx4: i32 =  tx3 + 68;
      let mut tdata5_3: i32 =  104;
      if (isGameLoop)
      {
        grayedOut = true;
        active = false;
      }
      if (isVidCom)
      {
        grayedOut = false;
        active = false;
      }
      if (this.game.EditObj.TestEarlyCinematics == 1 & isVidCom)
      {
        grayedOut = true;
        active = false;
        tdata5_3 = -1;
      }
      rectangle1 = this.DrawOneTab(g, active, tx4, "MNG", -1, grayedOut: grayedOut) with
      {
        Y = 36
      };
      rectangle1.Height -= 36;
      if (isVidCom)
        simpleStringList.Add("Click to go to to MANAGEMENT SCREEN mode. [Shortkey N]", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, tdata5_3, false);
      if (isGameLoop)
        simpleStringList.Add("Inaccesible during the AIs turn.", 0, rectangle1.X, rectangle1.Y, rectangle1.Width, rectangle1.Height, -1, false);
      let mut num1: i32 =  tx4 + 68;
      ref Graphics local1 = ref g;
      bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
      ref local2: Bitmap = ref bitmap;
      Rectangle rectangle2 = Rectangle::new(0, 140, 300, 63);
      let mut srcrect1: &Rectangle = &rectangle2
      Rectangle rectangle3 = Rectangle::new(0, 0, 300, 63);
      let mut destrect1: &Rectangle = &rectangle3
      DrawMod.DrawSimplePart2(ref local1, ref local2, srcrect1, destrect1);
      if (this.game.ScreenWidth > 2600)
      {
        let mut width: i32 =  (int) Math.Round( this.game.ScreenWidth / 2.0);
        ref Graphics local3 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
        ref local4: Bitmap = ref bitmap;
        rectangle3 = Rectangle::new(300, 140, width, 32);
        let mut srcrect2: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(300, 0, width, 32);
        let mut destrect2: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local3, ref local4, srcrect2, destrect2);
        ref Graphics local5 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
        ref local6: Bitmap = ref bitmap;
        rectangle3 = Rectangle::new(0, 140, width, 32);
        let mut srcrect3: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(300 + width, 0, width, 32);
        let mut destrect3: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local5, ref local6, srcrect3, destrect3);
        ref Graphics local7 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
        ref local8: Bitmap = ref bitmap;
        rectangle3 = Rectangle::new(screenWidth - width - 150, 140, 150, 32);
        let mut srcrect4: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(screenWidth - 150, 0, 150, 32);
        let mut destrect4: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect4, destrect4);
      }
      else
      {
        ref Graphics local9 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
        ref local10: Bitmap = ref bitmap;
        rectangle3 = Rectangle::new(300, 140, screenWidth - 440, 32);
        let mut srcrect5: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(300, 0, screenWidth - 440, 32);
        let mut destrect5: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect5, destrect5);
        ref Graphics local11 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCBOTBAR);
        ref local12: Bitmap = ref bitmap;
        num2: i32;
        rectangle3 = Rectangle::new(screenWidth - num2 - 150, 140, 150, 32);
        let mut srcrect6: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(screenWidth - 150, 0, 150, 32);
        let mut destrect6: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect6, destrect6);
      }
      for (let mut x: i32 =  290; x < this.game.ScreenWidth; x += 50)
      {
        ref Graphics local13 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.SE1_RESOURCEBAR_BOTTOM);
        ref local14: Bitmap = ref bitmap;
        rectangle3 = Rectangle::new(15, 22, 50, 20);
        let mut srcrect7: &Rectangle = &rectangle3
        rectangle2 = Rectangle::new(x, 22, 50, 20);
        let mut destrect7: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect7, destrect7);
      }
      ref Graphics local15 = ref g;
      bitmap = BitmapStore.GetBitmap(this.game.SE1_RESOURCEBAR_LEFT);
      ref local16: Bitmap = ref bitmap;
      DrawMod.DrawSimple(ref local15, ref local16, 0, 0);
      str: String = !(this.game.EditObj.RealTurn > -1 & this.game.EditObj.RealTurn <= this.game.Data.RegimeCounter & this.game.EditObj.RealRound > 0) ? "System Calculations" : this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Name;
      g.MeasureString(str, DrawMod.TGame.MarcFont16);
      DrawMod.DrawTextColouredConsoleCenter(ref g, str, this.game.MarcFont16, 193, 15, this.game.seColWhite);
      if (this.game.EditObj.RealRound > 0 && this.game.EditObj.RealTurn > -1 & this.game.EditObj.RealTurn <= this.game.Data.RegimeCounter)
      {
        let mut red: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Red;
        let mut green: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Green;
        let mut blue: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Blue;
        let mut red2: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Red2;
        let mut green2: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Green2;
        let mut blue2: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Blue2;
        let mut bannerSpriteNr: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].BannerSpriteNr;
        ref Graphics local17 = ref g;
        bitmap = BitmapStore.GetBitmap(bannerSpriteNr);
        ref local18: Bitmap = ref bitmap;
        double r1 =  ( red /  byte.MaxValue);
        double g1 =  ( green /  byte.MaxValue);
        double b1 =  ( blue /  byte.MaxValue);
        DrawMod.DrawScaledColorized2(ref local17, ref local18, 13, 15, 80, 60, 124, 210,  r1,  g1,  b1, 1f);
        let mut bannerSpriteNr2: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].BannerSpriteNr2;
        if (bannerSpriteNr2 > 0)
        {
          ref Graphics local19 = ref g;
          bitmap = BitmapStore.GetBitmap(bannerSpriteNr2);
          ref local20: Bitmap = ref bitmap;
          double r2 =  ( red2 /  byte.MaxValue);
          double g2 =  ( green2 /  byte.MaxValue);
          double b2 =  ( blue2 /  byte.MaxValue);
          DrawMod.DrawScaledColorized2(ref local19, ref local20, 13, 15, 80, 60, 124, 210,  r2,  g2,  b2, 1f);
        }
        let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[this.game.EditObj.RealTurn].HQSpriteNr2;
        if (hqSpriteNr2 > 0)
        {
          ref Graphics local21 = ref g;
          bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
          ref local22: Bitmap = ref bitmap;
          double r3 =  ( this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Red3 /  byte.MaxValue) - 1.0;
          double g3 =  ( this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Green3 /  byte.MaxValue) - 1.0;
          double b3 =  ( this.game.Data.RegimeObj[this.game.EditObj.RealTurn].Blue3 /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local21, ref local22, 30, 27,  r3,  g3,  b3, 0.95f);
        }
      }
      BitmapStore.GetBitmap(this.game.MARCTOPBAR).RotateFlip(RotateFlipType.Rotate180FlipX);
      for (let mut index: i32 =  0; index < this.game.ScreenWidth; index += 100)
      {
        ref Graphics local23 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.MARCTOPBAR);
        ref local24: Bitmap = ref bitmap;
        let mut x: i32 =  index;
        let mut y: i32 =  screenHeight - 37;
        DrawMod.DrawSimple(ref local23, ref local24, x, y);
      }
      BitmapStore.GetBitmap(this.game.MARCTOPBAR).RotateFlip(RotateFlipType.Rotate180FlipX);
      return simpleStringList;
    }

    pub Rectangle DrawOneTab(
      Graphics g,
      bool active,
      tx: i32,
      s: String,
      iconSlot: i32,
      let mut smallNumber: i32 =  -1,
      bool grayedOut = false,
      bool MousingOverNow = false)
    {
      let mut y1: i32 =  0;
      if (!active)
        y1 = -12;
      bitmap: Bitmap;
      if (MousingOverNow)
      {
        ref Graphics local1 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.SE1_RESOURCEBAR_TAB);
        ref local2: Bitmap = ref bitmap;
        let mut x: i32 =  tx;
        let mut y2: i32 =  y1;
        DrawMod.Draw(ref local1, ref local2, x, y2, 0.05f, 0.05f, 0.05f, 1f);
      }
      else
      {
        ref Graphics local3 = ref g;
        bitmap = BitmapStore.GetBitmap(this.game.SE1_RESOURCEBAR_TAB);
        ref local4: Bitmap = ref bitmap;
        let mut x: i32 =  tx;
        let mut y3: i32 =  y1;
        DrawMod.DrawSimple(ref local3, ref local4, x, y3);
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (iconSlot > -1)
      {
        if (grayedOut)
        {
          ref Graphics local5 = ref g;
          bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONS);
          ref local6: Bitmap = ref bitmap;
          rectangle1 = Rectangle::new(iconSlot * 42, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2Coloured(ref local5, ref local6, srcrect, destrect, 0.5f, 0.5f, 0.5f, 1f);
        }
        else if (MousingOverNow & !active)
        {
          ref Graphics local7 = ref g;
          bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONS);
          ref local8: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect, destrect);
        }
        else
        {
          if (!active)
          {
            ref Graphics local9 = ref g;
            bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONS);
            ref local10: Bitmap = ref bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 0, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
          }
          if (active)
          {
            ref Graphics local11 = ref g;
            bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONS);
            ref local12: Bitmap = ref bitmap;
            rectangle2 = Rectangle::new(iconSlot * 42, 32, 42, 32);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect, destrect);
          }
        }
      }
      c: Color;
      if (smallNumber > 0)
      {
        if (!active)
        {
          ref Graphics local13 = ref g;
          bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONHIGHLIGHT);
          ref local14: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 0, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect, destrect);
          c = Color.FromArgb((int) byte.MaxValue, 225, 215, 215);
        }
        if (active)
        {
          ref Graphics local15 = ref g;
          bitmap = BitmapStore.GetBitmap(this.game.SE1_ICONHIGHLIGHT);
          ref local16: Bitmap = ref bitmap;
          rectangle2 = Rectangle::new(0, 32, 42, 32);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx + 11, y1 + 20, 42, 32);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
          c = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 245, 245);
        }
        str: String = smallNumber.ToString();
        if (smallNumber > 9)
          str = "X";
        SizeF sizeF = g.MeasureString(str, DrawMod.TGame.MarcFont5);
        DrawMod.DrawTextColouredConsole(ref g, str, this.game.MarcFont5, tx + (int) Math.Round((68.0 -  sizeF.Width) / 2.0) + 11, y1 + 22, c);
      }
      SizeF sizeF1 = g.MeasureString(s, DrawMod.TGame.MarcFont16);
      if (active)
        c = this.game.seColWhite;
      if (!active)
        c = this.game.seColGray;
      if (grayedOut)
        c = Color.FromArgb((int) byte.MaxValue, 128, 128, 128);
      DrawMod.DrawTextColouredConsole(ref g, s, this.game.MarcFont16, tx + (int) Math.Round((68.0 -  sizeF1.Width) / 2.0), y1 + 48, c);
      Rectangle rectangle3 = Rectangle::new(tx, y1, 68, 75);
      tx += 68;
      return rectangle3;
    }

    pub fn DrawNumberWithDelta(Graphics g, x: i32, y: i32, texty: String, delta: i32)
    {
      let mut eventPic1: i32 =  DrawMod.TGame.Data.FindEventPic("", 8, "SE_Present");
      let mut eventPic2: i32 =  DrawMod.TGame.Data.FindEventPic("", 9, "SE_Present");
      let mut eventPic3: i32 =  DrawMod.TGame.Data.FindEventPic("", 11, "SE_Present");
      bool flag = false;
      SizeF sizeF1 = SizeF::new();
      let mut num: i32 =  delta;
      if (num < 0)
      {
        num = Math.Abs(num);
        flag = true;
      }
      tstring: String = num.ToString();
      SizeF sizeF2 = g.MeasureString(texty, DrawMod.TGame.MarcFont16);
      let mut index: i32 =  eventPic3;
      if (flag)
        index = eventPic2;
      else if (delta > 0)
        index = eventPic1;
      DrawMod.DrawTextColouredConsole(ref g, texty, DrawMod.TGame.se1TypeWriterMedium, x, y, DrawMod.TGame.seColTW);
      if (Strings.InStr(texty, "?") > 0 | Strings.InStr(texty.ToLower(), "unknown") > 0 || DrawMod.TGame.Data.Round == 1)
        return;
      x = (int) Math.Round( ( x + (sizeF2.Width + 8f)));
      ref Graphics local1 = ref g;
      bitmap: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index]);
      ref local2: Bitmap = ref bitmap;
      let mut x1: i32 =  x;
      let mut y1: i32 =  y + 2;
      DrawMod.DrawSimple(ref local1, ref local2, x1, y1);
      if (delta == 0)
        return;
      if (num >= 100000)
        tstring = Math.Floor( num / 1000.0).ToString() + "K";
      DrawMod.DrawTextColouredConsole(ref g, tstring, DrawMod.TGame.se1TypeWriterMedium, x + 16, y, DrawMod.TGame.seColTW);
    }

    pub DrawLeaderPortrait: Bitmap(
      charId: i32,
      w: i32,
      h: i32,
      bool showRelation = false,
      let mut relChange: i32 =  0,
      let mut isPeoplePortraitGroup: i32 =  -1,
      let mut isPeopleId: i32 =  -1,
      let mut isPeopleType: i32 =  -1,
      let mut isRegId: i32 =  -1,
      let mut isAllowHair: i32 =  -1,
      let mut isUniformEventPic: i32 =  -1,
      let mut sfNr: i32 =  -1,
      bool transBG = false)
    {
      libName: String = "SE_Data";
      let mut stringListById1: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 200, 0, 0));
      let mut stringListById2: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 196, 0, 0));
      DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 437, 0, 0));
      let mut stringListById3: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 438, 0, 0));
      let mut stringListById4: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 310, 0, 0));
      let mut stringListById5: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 306, 0, 0));
      let mut stringListById6: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(libName, 143, 0, 0));
      num1: i32;
      idValue1: i32;
      num2: i32;
      num3: i32;
      num4: i32;
      num5: i32;
      if (isPeoplePortraitGroup < 1)
      {
        num1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 16)));
        idValue1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 13)));
        num2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 5)));
        let mut num6: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 6)));
        num3 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 15)));
        num4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 20)));
        num5 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 26)));
      }
      if (num2 < 1 & num5 > 0)
        num2 = num5;
      if (isRegId > -1)
        num2 = isRegId;
      let mut regimeById: i32 =  this.game.HandyFunctionsObj.GetRegimeByID(num2);
      let mut idValue2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].GetData(0, num2, 2)));
      let mut idValue3: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].GetData(0, idValue2, 1)));
      let mut num7: i32 =  (int) Math.Round( (this.game.Data.Round - num1) / 6.0);
      Random random = new Random(charId);
      let mut num8: i32 =  random.Next(1, 3);
      let mut num9: i32 =  random.Next(30, 60);
      if (random.Next(0, 100) > 60)
        num9 = 0;
      else if (random.Next(0, 100) > 50)
        num9 = 999;
      if (isPeoplePortraitGroup > 0)
      {
        num7 = 30;
        if (isPeopleType == 1)
          num7 = 18;
        if (isPeopleType == 2)
          num7 = 32;
        if (isPeopleType == 3)
          num7 = 50;
        if (isPeopleType == 4)
          num7 = 60;
        if (isPeopleType == 12)
          num7 = 32;
        if (isPeopleType == 13)
          num7 = 50;
        if (isPeopleType == 14)
          num7 = 60;
        num9 = 50;
        if (isAllowHair < 1)
          num9 = 10;
        num8 = 2;
      }
      objBitmap1: Bitmap = new Bitmap(100, 140, PixelFormat.Format32bppPArgb);
      objBitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics objGraphics1 = Graphics.FromImage((Image) objBitmap1);
      objGraphics1.Clear(Color.Transparent);
      objBitmap2: Bitmap = new Bitmap(100, 140, PixelFormat.Format32bppPArgb);
      objBitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics objGraphics2 = Graphics.FromImage((Image) objBitmap2);
      if (isUniformEventPic > 0 && isPeoplePortraitGroup < 1)
        isPeoplePortraitGroup = 9999;
      bitmap1: Bitmap;
      if (isPeoplePortraitGroup > 0 | transBG)
        objGraphics2.Clear(Color.Transparent);
      else if (relChange == -999)
      {
        DrawMod.DrawBlock(ref objGraphics1, 0, 0, w, h, 0, 0, 0, (int) byte.MaxValue);
        relChange = 0;
      }
      else if (regimeById != this.game.Data.Turn)
      {
        if (regimeById > -1)
        {
          float num10 =  this.game.Data.RegimeObj[regimeById].Red /  byte.MaxValue;
          float num11 =  this.game.Data.RegimeObj[regimeById].Green /  byte.MaxValue;
          float num12 =  this.game.Data.RegimeObj[regimeById].Blue /  byte.MaxValue;
          ref Graphics local1 = ref objGraphics2;
          bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_PORTRAITBACKGROUND);
          ref local2: Bitmap = ref bitmap2;
          double r =  num10 - 1.0;
          double g =  num11 - 1.0;
          double b =  num12 - 1.0;
          DrawMod.Draw(ref local1, ref local2, 0, 0,  r,  g,  b, 1f);
          ref Graphics local3 = ref objGraphics2;
          bitmap1 = BitmapStore.GetBitmap(this.game.Data.RegimeObj[regimeById].SymbolSpriteNr);
          ref local4: Bitmap = ref bitmap1;
          let mut width: i32 =  BitmapStore.GetWidth(this.game.Data.RegimeObj[this.game.Data.Turn].SymbolSpriteNr);
          let mut origh: i32 =  BitmapStore.Getheight(this.game.Data.RegimeObj[this.game.Data.Turn].SymbolSpriteNr);
          DrawMod.DrawScaledColorized(ref local3, ref local4, -20, 0, 140, 140, width, origh, 1f, 1f, 1f, 0.12f);
        }
        else
        {
          ref Graphics local5 = ref objGraphics2;
          bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_PORTRAITBACKGROUND);
          ref local6: Bitmap = ref bitmap3;
          DrawMod.DrawSimple(ref local5, ref local6, 0, 0);
        }
      }
      else if (idValue1 > 0)
      {
        float num13 =  (Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue1, 7)) / 512.0) + 0.5f;
        float num14 =  (Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue1, 8)) / 512.0) + 0.5f;
        float num15 =  (Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue1, 9)) / 512.0) + 0.5f;
        if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue1, 6))) == charId)
        {
          ref Graphics local7 = ref objGraphics2;
          bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_PORTRAITBACKGROUNDFACTIONLEADER);
          ref local8: Bitmap = ref bitmap4;
          double r =  (num13 - 0.75f);
          double g =  (num14 - 0.75f);
          double b =  (num15 - 0.75f);
          DrawMod.Draw(ref local7, ref local8, 0, 0,  r,  g,  b, 1f);
        }
        else
        {
          ref Graphics local9 = ref objGraphics2;
          bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_PORTRAITBACKGROUND);
          ref local10: Bitmap = ref bitmap5;
          double r =  (num13 - 0.75f);
          double g =  (num14 - 0.75f);
          double b =  (num15 - 0.75f);
          DrawMod.Draw(ref local9, ref local10, 0, 0,  r,  g,  b, 1f);
        }
      }
      else
      {
        ref Graphics local11 = ref objGraphics2;
        bitmap1 = BitmapStore.GetBitmap(DrawMod.TGame.SE1_PORTRAITBACKGROUND);
        ref local12: Bitmap = ref bitmap1;
        DrawMod.DrawSimple(ref local11, ref local12, 0, 0);
      }
      bool flag1 = false;
      let mut num16: i32 =  1;
      Rectangle rectangle1;
      Rectangle rectangle2;
      do
      {
        returnCol: i32;
        if (num16 == 1)
          returnCol = 61;
        if (num16 == 2)
          returnCol = 59;
        if (num16 == 3)
          returnCol = 57;
        if (num16 == 4)
          returnCol = 55;
        if (num16 == 5)
          returnCol = 53;
        idValue4: i32;
        num17: i32;
        index1: i32;
        if (isPeoplePortraitGroup < 1)
        {
          idValue4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, returnCol)));
          num17 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, returnCol + 1)));
        }
        else
        {
          SimpleList simpleList = SimpleList::new();
          let mut length: i32 =  this.game.Data.StringListObj[stringListById3].Length;
          for (let mut index2: i32 =  0; index2 <= length; index2 += 1)
          {
            if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 1])) == isPeoplePortraitGroup && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 2])) == num16)
            {
              index1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 3]));
              let mut num18: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 5]));
              let mut num19: i32 =  (int) Math.Round(Math.Floor( BitmapStore.Getheight(this.game.Data.EventPicNr[index1]) /  (num18 + 1)));
              for (let mut tdata2: i32 =  1; tdata2 <= num19; tdata2 += 1)
                simpleList.Add((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 0])) * 10 + tdata2, 10, (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index2, 0])), tdata2);
            }
          }
          let mut index3: i32 =  num16 != 4 ? simpleList.GetRandomSlotbasedOnWeightWithSeed( (isPeopleId + this.game.Data.GameID * num16)) : simpleList.GetRandomSlotbasedOnWeightWithSeed( (isRegId + this.game.Data.GameID * num16));
          if (index3 > -1)
          {
            idValue4 = simpleList.Data1[index3];
            num17 = simpleList.Data2[index3] - 1;
          }
        }
        y: i32;
        height: i32;
        float num20;
        float num21;
        if (idValue4 > 0)
        {
          let mut num22: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, idValue4, 1)));
          let mut num23: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, idValue4, 2)));
          index1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, idValue4, 3)));
          y = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, idValue4, 4)));
          height = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, idValue4, 5)));
          num20 = 1f;
          num21 = 0.0f;
          if (num7 > 25 & num16 == 2 & num8 == 1)
          {
            num20 = 0.0f;
            num21 = 1f;
          }
          if (num7 > 33 & num16 == 3 & num8 == 1)
          {
            num20 = 0.0f;
            num21 = 1f;
          }
          if (num7 > 25 & num16 == 3 & num8 == 2)
          {
            num20 = 0.0f;
            num21 = 1f;
          }
          if (num7 > 33 & num16 == 2 & num8 == 2)
          {
            num20 = 0.0f;
            num21 = 1f;
          }
          if (num16 == 4 & num7 > 40 & num7 < 65)
            num21 =  (num7 - 40) / 25f;
          else if (num16 == 4 & num7 >= 65)
          {
            num21 = 1f;
            num20 = 0.0f;
          }
        }
        if (idValue4 > 0 | num16 == 5 && num16 == 5)
        {
          float saturation = 1f;
          if (num7 >= 15 & num7 < 40)
            saturation =  (1.35 - 0.35 * ( (num7 - 15) / 25.0));
          else if (num7 >= 50)
          {
            saturation =  (1.0 -  (num7 - 50) / 200.0);
            if ( saturation < 0.9)
              saturation = 0.8f;
          }
          if ( saturation != 1.0)
            DrawMod.DrawSaturized(ref objGraphics2, ref objBitmap1, 0, 0, saturation);
          else
            DrawMod.DrawSimple(ref objGraphics2, ref objBitmap1, 0, 0);
          DrawMod.DrawSimple(ref objGraphics1, ref objBitmap2, 0, 0);
        }
        if (idValue4 > 0)
        {
          if (num3 == 1 & num8 == 1 | isPeoplePortraitGroup > 0 & isAllowHair < 1)
          {
            if (num16 == 5 & num7 > num9)
            {
              num21 = 0.0f;
              num20 = 0.0f;
            }
          }
          else if (num16 == 5 & num7 > 50 & num7 < 65)
            num21 =  (num7 - 50) / 15f;
          else if (num16 == 5 & num7 >= 65)
          {
            num21 = 1f;
            num20 = 0.0f;
          }
          if (num16 == 1 & num7 < 60)
            num21 =  num7 / 60f;
          else if (num16 == 1 & num7 >= 60)
          {
            num21 = 1f;
            num20 = 0.0f;
          }
          if ( num20 == 1.0)
          {
            ref Graphics local13 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index1]);
            ref local14: Bitmap = ref bitmap1;
            rectangle1 = Rectangle::new(0, (height + 1) * num17, 100, height);
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(0, y, 100, height);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect, destrect);
          }
          else if ( num20 > 0.0)
          {
            ref Graphics local15 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index1]);
            ref local16: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(0, (height + 1) * num17, 100, height);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, y, 100, height);
            let mut destrect: &Rectangle = &rectangle1
            double a =  num20;
            DrawMod.DrawSimplePart2Coloured(ref local15, ref local16, srcrect, destrect, 0.0f, 0.0f, 0.0f,  a);
          }
          if ( num21 == 1.0)
          {
            ref Graphics local17 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index1]);
            ref local18: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(101, (height + 1) * num17, 100, height);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, y, 100, height);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect, destrect);
          }
          else if ( num21 > 0.0)
          {
            ref Graphics local19 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index1]);
            ref local20: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(101, (height + 1) * num17, 100, height);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, y, 100, height);
            let mut destrect: &Rectangle = &rectangle1
            double a =  num21;
            DrawMod.DrawSimplePart2Coloured(ref local19, ref local20, srcrect, destrect, 0.0f, 0.0f, 0.0f,  a);
          }
          flag1 = true;
        }
        num16 += 1;
      }
      while (num16 <= 5);
      if (flag1 | isPeoplePortraitGroup == 9999)
      {
        if (isUniformEventPic > -1)
        {
          let mut num24: i32 =  0;
          if (isPeopleType == 2)
            num24 = 1;
          if (isPeopleType == 3)
            num24 = 2;
          if (isPeopleType == 4)
            num24 = 3;
          if (isPeopleType == 13)
            num24 = 1;
          if (isPeopleType == 14)
            num24 = 2;
          bool flag2 = false;
          let mut index4: i32 =  isUniformEventPic;
          if (BitmapStore.Getheight(this.game.Data.EventPicNr[index4]) >= 280)
            flag2 = true;
          let mut num25: i32 =  (int) Math.Round(Math.Floor( BitmapStore.GetWidth(this.game.Data.EventPicNr[index4]) / 100.0));
          let mut num26: i32 =  num24;
          if (BitmapStore.GetWidth(this.game.Data.EventPicNr[index4]) < (num26 + 1) * 100)
            num26 = 0;
          ref Graphics local21 = ref objGraphics1;
          bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index4]);
          ref local22: Bitmap = ref bitmap1;
          rectangle2 = Rectangle::new(num26 * 100, 0, 100, 140);
          let mut srcrect1: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, 0, 100, 140);
          let mut destrect1: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local21, ref local22, srcrect1, destrect1);
          if (flag2 & regimeById > -1)
          {
            float num27 =  this.game.Data.RegimeObj[regimeById].Red /  byte.MaxValue;
            float num28 =  this.game.Data.RegimeObj[regimeById].Green /  byte.MaxValue;
            float num29 =  this.game.Data.RegimeObj[regimeById].Blue /  byte.MaxValue;
            bool flag3 = false;
            if (sfNr > -1)
            {
              if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sfNr].Type].Theater == 2)
                flag3 = true;
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
              {
                let mut sfCount: i32 =  this.game.Data.UnitObj[index5].SFCount;
                for (let mut index6: i32 =  0; index6 <= sfCount; index6 += 1)
                {
                  if (this.game.Data.UnitObj[index5].SFList[index6] == sfNr && this.game.Data.UnitObj[index5].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index5].Historical].TempVar1 == 1)
                    flag3 = true;
                }
              }
            }
            if (flag3)
            {
              num27 =  this.game.Data.RegimeObj[regimeById].Red2 /  byte.MaxValue;
              num28 =  this.game.Data.RegimeObj[regimeById].Green2 /  byte.MaxValue;
              num29 =  this.game.Data.RegimeObj[regimeById].Blue2 /  byte.MaxValue;
            }
            ref Graphics local23 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index4]);
            ref local24: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num26 * 100, 140, 100, 140);
            let mut srcrect2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, 100, 140);
            let mut destrect2: &Rectangle = &rectangle1
            double r =  (-0.0f + num27);
            double g =  (-0.0f + num28);
            double b =  (-0.0f + num29);
            DrawMod.DrawSimplePart2ColouredNew(ref local23, ref local24, srcrect2, destrect2,  r,  g,  b, 1f);
          }
        }
        else
        {
          let mut num30: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 6)));
          let mut returnCol: i32 =  7;
          if (num30 == 3 | num30 == 4)
            returnCol = 9;
          if (num30 == 5 | num30 == 6 | num30 == 10)
            returnCol = 8;
          let mut index: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById4].GetData(0, idValue3, returnCol)));
          if (index > 0)
          {
            bool flag4 = false;
            if (BitmapStore.Getheight(this.game.Data.EventPicNr[index]) >= 280)
              flag4 = true;
            let mut num31: i32 =  (int) Math.Round(Math.Floor( BitmapStore.GetWidth(this.game.Data.EventPicNr[index]) / 100.0));
            let mut num32: i32 =  0;
            if (num31 > 1)
              num32 = new Random(charId).Next(1, num31 + 1) - 1;
            ref Graphics local25 = ref objGraphics1;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index]);
            ref local26: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num32 * 100, 0, 100, 140);
            let mut srcrect3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, 100, 140);
            let mut destrect3: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local25, ref local26, srcrect3, destrect3);
            if (flag4 & regimeById > -1)
            {
              float num33 =  this.game.Data.RegimeObj[regimeById].Red /  byte.MaxValue;
              float num34 =  this.game.Data.RegimeObj[regimeById].Green /  byte.MaxValue;
              float num35 =  this.game.Data.RegimeObj[regimeById].Blue /  byte.MaxValue;
              bool flag5 = false;
              if (charId > -1)
              {
                num32 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 6)));
                let mut id: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, charId, 7)));
                if (num32 == 3 & id > 0)
                {
                  let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(id);
                  if (historicalUnitById > -1 && this.game.Data.HistoricalUnitObj[historicalUnitById].TempVar1 == 1)
                    flag5 = true;
                }
              }
              if (flag5)
              {
                num33 =  this.game.Data.RegimeObj[regimeById].Red2 /  byte.MaxValue;
                num34 =  this.game.Data.RegimeObj[regimeById].Green2 /  byte.MaxValue;
                num35 =  this.game.Data.RegimeObj[regimeById].Blue2 /  byte.MaxValue;
              }
              ref Graphics local27 = ref objGraphics1;
              bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index]);
              ref local28: Bitmap = ref bitmap1;
              rectangle2 = Rectangle::new(num32 * 100, 140, 100, 140);
              let mut srcrect4: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(0, 0, 100, 140);
              let mut destrect4: &Rectangle = &rectangle1
              double r =  (-0.0f + num33);
              double g =  (-0.0f + num34);
              double b =  (-0.0f + num35);
              DrawMod.DrawSimplePart2ColouredNew(ref local27, ref local28, srcrect4, destrect4,  r,  g,  b, 1f);
            }
          }
        }
      }
      objGraphics1.Dispose();
      objGraphics2.Dispose();
      objGraphics2 = (Graphics) null;
      if (isPeoplePortraitGroup < 1)
      {
        if (relChange > 100 | relChange < -100)
          showRelation = false;
        else if (charId < 1)
          showRelation = false;
      }
      else
        showRelation = false;
      if (w == 100 & h == 140 & !showRelation)
        return objBitmap1;
      bitmap6: Bitmap = new Bitmap(w, h, PixelFormat.Format32bppPArgb);
      bitmap6.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics = Graphics.FromImage((Image) bitmap6);
      DrawMod.DrawScaled(ref graphics, ref objBitmap1, 0, 0, w, h, true);
      if (showRelation)
      {
        if (regimeById != this.game.Data.Turn & regimeById != -1)
        {
          num4 = relChange;
          if (relChange == 0)
            num4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0))].GetData3(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, this.game.Data.RegimeObj[regimeById].id, 2, "relation", 3)));
          relChange = 0;
        }
        color1: Color = num4 < 75 ? (num4 < 50 ? (num4 < 25 ? this.game.seColRed : this.game.seColBlue) : this.game.seColYellow) : this.game.seColGreen;
        c1: Color = DrawMod.LightenColor(color1, -150);
        color2: Color = DrawMod.LightenColor(color1, 75);
        c2: Color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColGreen.R) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColGreen.G) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColGreen.B) / 2.0));
        c3: Color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColRed.R) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColRed.G) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) this.game.seColRed.B) / 2.0));
        c4: Color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( ((int) byte.MaxValue + (int) color2.R) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) color2.G) / 2.0), (int) Math.Round( ((int) byte.MaxValue + (int) color2.B) / 2.0));
        if (h <= 110)
        {
          DrawMod.DrawBlock(ref graphics, 3, h - 12, w - 6, 6, 0, 0, 0, 128);
          DrawMod.DrawBlockGradient(ref graphics, 3, h - 12, (int) Math.Round( (w - 6) * ( num4 / 100.0)), 6, c1, color1);
          DrawMod.DrawTextColouredConsole(ref graphics, num4.ToString(), this.game.MarcFont7, 7, h - 16, Color.FromArgb(128, 0, 0, 0));
          DrawMod.DrawTextColouredConsole(ref graphics, num4.ToString(), this.game.MarcFont7, 6, h - 17, c4);
        }
        else
        {
          DrawMod.DrawBlock(ref graphics, 4, h - 16, w - 8, 8, 0, 0, 0, 128);
          DrawMod.DrawBlockGradient(ref graphics, 4, h - 16, (int) Math.Round( (w - 8) * ( num4 / 100.0)), 8, c1, color1);
          DrawMod.DrawTextColouredConsole(ref graphics, num4.ToString(), this.game.MarcFont16, 9, h - 20, Color.FromArgb(128, 0, 0, 0));
          DrawMod.DrawTextColouredConsole(ref graphics, num4.ToString(), this.game.MarcFont16, 8, h - 21, c4);
        }
        if (relChange != 0)
        {
          SizeF sizeF1 = SizeF::new();
          if (relChange > 0)
          {
            SizeF sizeF2 = graphics.MeasureString("+" + relChange.ToString(), this.game.MarcFont16);
            DrawMod.DrawBlock(ref graphics, (int) Math.Round( w - (3.0 +  sizeF2.Width) + 1.0), h - 18, (int) Math.Round( (sizeF2.Width + 2f)), 14, 0, 0, 0, 128);
            DrawMod.DrawTextColouredConsole(ref graphics, "+" + relChange.ToString(), this.game.MarcFont16, (int) Math.Round( w - (3.0 +  sizeF2.Width) + 1.0), h - 17, Color.FromArgb(128, 0, 0, 0));
            DrawMod.DrawTextColouredConsole(ref graphics, "+" + relChange.ToString(), this.game.MarcFont16, (int) Math.Round( ( w - (3f + sizeF2.Width))), h - 18, c2);
          }
          else if (relChange < 0)
          {
            SizeF sizeF3 = graphics.MeasureString(relChange.ToString(), this.game.MarcFont7);
            DrawMod.DrawBlock(ref graphics, (int) Math.Round( w - (3.0 +  sizeF3.Width) + 1.0), h - 18, (int) Math.Round( (sizeF3.Width + 2f)), 14, 0, 0, 0, 128);
            DrawMod.DrawTextColouredConsole(ref graphics, relChange.ToString(), this.game.MarcFont16, (int) Math.Round( w - (3.0 +  sizeF3.Width) + 1.0), h - 17, Color.FromArgb(128, 0, 0, 0));
            DrawMod.DrawTextColouredConsole(ref graphics, relChange.ToString(), this.game.MarcFont16, (int) Math.Round( ( w - (3f + sizeF3.Width))), h - 18, c3);
          }
        }
      }
      if (regimeById != this.game.Data.Turn & isPeopleType == -1 && regimeById > 0)
      {
        let mut num36: i32 =  (int) Math.Round( w * 0.4);
        ref Graphics local29 = ref graphics;
        bitmap1 = BitmapStore.GetBitmap(this.game.Data.RegimeObj[regimeById].SymbolSpriteNr);
        ref local30: Bitmap = ref bitmap1;
        let mut x: i32 =  w - num36;
        let mut y: i32 =  h - num36;
        let mut w1: i32 =  num36;
        let mut h1: i32 =  num36;
        let mut width: i32 =  BitmapStore.GetWidth(this.game.Data.RegimeObj[regimeById].SymbolSpriteNr);
        let mut origh: i32 =  BitmapStore.Getheight(this.game.Data.RegimeObj[regimeById].SymbolSpriteNr);
        DrawMod.DrawScaledColorized(ref local29, ref local30, x, y, w1, h1, width, origh, 1f, 1f, 1f, 0.6f);
      }
      graphics.Dispose();
      graphics = (Graphics) null;
      objBitmap1.Dispose();
      objBitmap1 = (Bitmap) null;
      return bitmap6;
    }

    pub DrawActionCard: Bitmap(nr: i32, let mut roundnr: i32 =  -1, bool small = false)
    {
      bitmap1: Bitmap = small ? new Bitmap(40, 60, PixelFormat.Format32bppPArgb) : new Bitmap(300, 450, PixelFormat.Format32bppPArgb);
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) bitmap1);
      if (!small)
      {
        if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD1, 0, 0);
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD2, 0, 0);
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD3, 0, 0);
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD4, 0, 0);
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD5, 0, 0);
        else
          DrawMod.DrawSimple(ref Expression, ref this.game.CARD5, 0, 0);
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD1, 0, 0, 40, 60);
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD2, 0, 0, 40, 60);
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD3, 0, 0, 40, 60);
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD4, 0, 0, 40, 60);
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD5, 0, 0, 40, 60);
      else
        DrawMod.DrawScaled(ref Expression, ref this.game.CARD5, 0, 0, 40, 60);
      if (this.game.Data.ActionCardObj[nr].EventPicNr > -1)
      {
        if (!small)
        {
          ref Graphics local1 = ref Expression;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
          ref local2: Bitmap = ref bitmap2;
          DrawMod.DrawScaled(ref local1, ref local2, 17, 66, 260, 194);
          ref Graphics local3 = ref Expression;
          bitmap3: Bitmap = BitmapStore.GetBitmap(this.game.ACTIONFRAME);
          ref local4: Bitmap = ref bitmap3;
          DrawMod.DrawSimple(ref local3, ref local4, 17, 66);
        }
        else
        {
          if (this.game.Data.ActionCardObj[nr].Nato > -1)
          {
            ref Graphics local5 = ref Expression;
            bitmap4: Bitmap = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.ActionCardObj[nr].Nato]);
            ref local6: Bitmap = ref bitmap4;
            DrawMod.DrawScaled(ref local5, ref local6, 3, 11, 34, 22);
          }
          else
          {
            ref Graphics local7 = ref Expression;
            bitmap5: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local8: Bitmap = ref bitmap5;
            DrawMod.DrawScaled(ref local7, ref local8, 3, 11, 34, 22);
          }
          DrawMod.DrawRectangle(ref Expression, 4, 4, 12, 1, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref Expression, 4, 37, 29, 1, 0, 0, 0, (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref Expression, 4, 41, 27, 1, 0, 0, 0, (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref Expression, 4, 45, 14, 1, 0, 0, 0, (int) byte.MaxValue);
        }
      }
      if (!small)
      {
        SizeF sizeF = SizeF::new();
        if ( Expression.MeasureString(this.game.Data.ActionCardObj[nr].Title, this.game.VicFont1).Width < 200.0)
          DrawMod.DrawTextVic2(ref Expression, this.game.Data.ActionCardObj[nr].Title, this.game.VicFont1, 21, 30, this.game.VicColor2, this.game.VicColor2Shade);
        else if ( Expression.MeasureString(this.game.Data.ActionCardObj[nr].Title, this.game.VicFont2).Width < 200.0)
          DrawMod.DrawTextVic2(ref Expression, this.game.Data.ActionCardObj[nr].Title, this.game.VicFont2, 21, 30, this.game.VicColor2, this.game.VicColor2Shade);
        else
          DrawMod.DrawTextVic2(ref Expression, this.game.Data.ActionCardObj[nr].Title, this.game.VicFont3, 21, 30, this.game.VicColor2, this.game.VicColor2Shade);
        tstring: String = Strings.Trim(Conversion.Str( this.game.Data.ActionCardObj[nr].PPCost));
        if (this.game.Data.ActionCardObj[nr].PPCost == -1)
          tstring = "N/A";
        DrawMod.DrawTextVic2(ref Expression, tstring, this.game.VicFont1, 223, 30, this.game.VicColor2, this.game.VicColor2Shade);
        str1: String = this.game.Data.ActionCardObj[nr].Text;
        let mut num1: i32 =  1;
        while (num1 == 1)
        {
          num1 = 0;
          let mut num2: i32 =  Strings.InStr(str1, "[gamevar]");
          if (num2 > 0)
          {
            let mut num3: i32 =  Strings.InStr(str1, "[/gamevar]");
            if (num3 > num2 & num3 > 0)
            {
              str2: String = Strings.Trim(Conversion.Str( this.game.Data.GameSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num2 + Strings.Len("[gamevar]"), num3 - (num2 + Strings.Len("[gamevar]")))))]));
              str1 = Strings.Left(str1, num2 - 1) + str2 + Strings.Mid(str1, num3 + Strings.Len("[/gamevar]"));
              num1 = 1;
            }
          }
          let mut num4: i32 =  Strings.InStr(str1, "[tempvar]");
          if (num4 > 0)
          {
            let mut num5: i32 =  Strings.InStr(str1, "[/tempvar]");
            if (num5 > num4 & num5 > 0)
            {
              str3: String = Strings.Trim(Conversion.Str( this.game.Data.TempVar[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num4 + Strings.Len("[tempvar]"), num5 - (num4 + Strings.Len("[tempvar]")))))]));
              str1 = Strings.Left(str1, num4 - 1) + str3 + Strings.Mid(str1, num5 + Strings.Len("[/tempvar]"));
              num1 = 1;
            }
          }
          let mut num6: i32 =  Strings.InStr(str1, "[regimevar]");
          if (num6 > 0)
          {
            let mut num7: i32 =  Strings.InStr(str1, "[/regimevar]");
            if (num7 > num6 & num7 > 0)
            {
              str4: String = Strings.Mid(str1, num6 + Strings.Len("[regimevar]"), num7 - (num6 + Strings.Len("[regimevar]")));
              let mut num8: i32 =  Strings.InStr(str4, ",");
              if (num8 > 0)
              {
                str5: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Left(str4, num8 - 1)))].RegimeSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str4, num8 + 1)))]));
                str1 = Strings.Left(str1, num6 - 1) + str5 + Strings.Mid(str1, num7 + Strings.Len("[/regimevar]"));
                num1 = 1;
              }
            }
          }
        }
        let mut num9: i32 =  1;
        while (num9 == 1)
        {
          num9 = 0;
          let mut num10: i32 =  Strings.InStr(str1, "[regimename]");
          if (num10 > 0)
          {
            let mut num11: i32 =  Strings.InStr(str1, "[/regimename]");
            if (num11 > num10 & num11 > 0)
            {
              name: String = this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num10 + Strings.Len("[regimename]"), num11 - (num10 + Strings.Len("[regimename]")))))].Name;
              str1 = Strings.Left(str1, num10 - 1) + name + Strings.Mid(str1, num11 + Strings.Len("[/regimename]"));
              num9 = 1;
            }
          }
          let mut num12: i32 =  Strings.InStr(str1, "[unitname]");
          if (num12 > 0)
          {
            let mut num13: i32 =  Strings.InStr(str1, "[/unitname]");
            if (num13 > num12 & num13 > 0)
            {
              name: String = this.game.Data.UnitObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num12 + Strings.Len("[unitname]"), num13 - (num12 + Strings.Len("[unitname]")))))].Name;
              str1 = Strings.Left(str1, num12 - 1) + name + Strings.Mid(str1, num13 + Strings.Len("[/unitname]"));
              num9 = 1;
            }
          }
          let mut num14: i32 =  Strings.InStr(str1, "[hexname]");
          if (num14 > 0)
          {
            let mut num15: i32 =  Strings.InStr(str1, "[/hexname]");
            if (num15 > num14 & num15 > 0)
            {
              str6: String = Strings.Mid(str1, num14 + Strings.Len("[hexname]"), num15 - (num14 + Strings.Len("[hexname]")));
              let mut num16: i32 =  Strings.InStr(str6, ",");
              if (num16 > 0)
              {
                hexName: String = this.game.HandyFunctionsObj.GetHexName((int) Math.Round(Conversion.Val(Strings.Left(str6, num16 - 1))), (int) Math.Round(Conversion.Val(Strings.Mid(str6, num16 + 1))), 0);
                str1 = Strings.Left(str1, num14 - 1) + hexName + Strings.Mid(str1, num15 + Strings.Len("[/hexname]"));
                num9 = 1;
              }
            }
          }
        }
        str7: String = str1;
        TextAreaClass textAreaClass = Strings.Len(str7) >= 75 ? (Strings.Len(str7) >= 125 ? (Strings.Len(str7) >= 200 ? new TextAreaClass(this.game, 282, 10, this.game.VicFont3, "", false, str7, Color.Black, tbackbitmap: (ref bitmap1), bbx: 15, bby: 275, tHideShade: true) : new TextAreaClass(this.game, 282, 10, this.game.VicFont2, "", false, str7, Color.Black, tItemSize: 20, tbackbitmap: (ref bitmap1), bbx: 15, bby: 275, tHideShade: true)) : new TextAreaClass(this.game, 282, 10, this.game.VicFont2, "", false, str7, Color.Black, tItemSize: 20, tbackbitmap: (ref bitmap1), bbx: 15, bby: 275, tHideShade: true)) : new TextAreaClass(this.game, 282, 10, this.game.VicFont2, "", false, str7, Color.Black, tItemSize: 20, tbackbitmap: (ref bitmap1), bbx: 15, bby: 275, tHideShade: true);
        ref Graphics local9 = ref Expression;
        bitmap6: Bitmap = textAreaClass.Paint();
        ref local10: Bitmap = ref bitmap6;
        DrawMod.DrawSimple(ref local9, ref local10, 15, 275);
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return bitmap1;
    }

    pub DrawActionCardMarc: Bitmap(
      nr: i32,
      let mut roundnr: i32 =  -1,
      let mut size: i32 =  1,
      bool Shaded = false,
      let mut Percent: i32 =  0)
    {
      bitmap1: Bitmap;
      switch (size)
      {
        case 1:
          bitmap1 = new Bitmap(190, 266, PixelFormat.Format32bppPArgb);
          break;
        case 2:
          bitmap1 = new Bitmap(105, 147, PixelFormat.Format32bppPArgb);
          break;
        default:
          bitmap1 = new Bitmap(33, 46, PixelFormat.Format32bppPArgb);
          break;
      }
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) bitmap1);
      c: Color = this.game.Data.ActionCardObj[nr].ColorScheme > 0 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 1 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 2 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 3 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 4 ? Color.White : Color.White) : Color.White) : Color.White) : Color.White) : Color.White;
      bitmap2: Bitmap;
      switch (size)
      {
        case 1:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local1 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1A);
            ref local2: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local1, ref local2, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local3 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2A);
            ref local4: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local3, ref local4, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local5 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3A);
            ref local6: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local5, ref local6, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local7 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4A);
            ref local8: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local7, ref local8, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local9 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5A);
            ref local10: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local9, ref local10, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local11 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6A);
            ref local12: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local11, ref local12, 0, 0);
            break;
          }
          ref Graphics local13 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5A);
          ref local14: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local13, ref local14, 0, 0);
          break;
        case 2:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local15 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1B);
            ref local16: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local15, ref local16, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local17 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2B);
            ref local18: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local17, ref local18, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local19 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3B);
            ref local20: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local19, ref local20, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local21 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4B);
            ref local22: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local21, ref local22, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local23 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5B);
            ref local24: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local23, ref local24, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local25 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6B);
            ref local26: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local25, ref local26, 0, 0);
            break;
          }
          ref Graphics local27 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5B);
          ref local28: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local27, ref local28, 0, 0);
          break;
        default:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local29 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1C);
            ref local30: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local29, ref local30, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local31 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2C);
            ref local32: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local31, ref local32, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local33 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3C);
            ref local34: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local33, ref local34, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local35 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4C);
            ref local36: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local35, ref local36, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local37 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5C);
            ref local38: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local37, ref local38, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local39 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6C);
            ref local40: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local39, ref local40, 0, 0);
            break;
          }
          ref Graphics local41 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5C);
          ref local42: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local41, ref local42, 0, 0);
          break;
      }
      if (this.game.Data.ActionCardObj[nr].EventPicNr > -1)
      {
        switch (size)
        {
          case 1:
            ref Graphics local43 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local44: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local43, ref local44, 8, 37, 165, 76);
            break;
          case 2:
            ref Graphics local45 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local46: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local45, ref local46, 4, 21, 92, 41);
            break;
          default:
            if (this.game.Data.ActionCardObj[nr].SmallGfx > -1)
            {
              ref Graphics local47 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[this.game.Data.ActionCardObj[nr].SmallGfx]);
              ref local48: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local47, ref local48, 1, 6, 29, 14);
              break;
            }
            if (this.game.Data.ActionCardObj[nr].Nato > -1)
            {
              ref Graphics local49 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.ActionCardObj[nr].Nato]);
              ref local50: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local49, ref local50, 1, 6, 29, 14);
              break;
            }
            ref Graphics local51 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local52: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local51, ref local52, 1, 6, 29, 14);
            break;
        }
      }
      str1: String = this.game.Data.ActionCardObj[nr].Text;
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 =  Strings.InStr(str1, "[gamevar]");
        if (num2 > 0)
        {
          let mut num3: i32 =  Strings.InStr(str1, "[/gamevar]");
          if (num3 > num2 & num3 > 0)
          {
            str2: String = Strings.Trim(Conversion.Str( this.game.Data.GameSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num2 + Strings.Len("[gamevar]"), num3 - (num2 + Strings.Len("[gamevar]")))))]));
            str1 = Strings.Left(str1, num2 - 1) + str2 + Strings.Mid(str1, num3 + Strings.Len("[/gamevar]"));
            num1 = 1;
          }
        }
        let mut num4: i32 =  Strings.InStr(str1, "[tempvar]");
        if (num4 > 0)
        {
          let mut num5: i32 =  Strings.InStr(str1, "[/tempvar]");
          if (num5 > num4 & num5 > 0)
          {
            str3: String = Strings.Trim(Conversion.Str( this.game.Data.TempVar[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num4 + Strings.Len("[tempvar]"), num5 - (num4 + Strings.Len("[tempvar]")))))]));
            str1 = Strings.Left(str1, num4 - 1) + str3 + Strings.Mid(str1, num5 + Strings.Len("[/tempvar]"));
            num1 = 1;
          }
        }
        let mut num6: i32 =  Strings.InStr(str1, "[regimevar]");
        if (num6 > 0)
        {
          let mut num7: i32 =  Strings.InStr(str1, "[/regimevar]");
          if (num7 > num6 & num7 > 0)
          {
            str4: String = Strings.Mid(str1, num6 + Strings.Len("[regimevar]"), num7 - (num6 + Strings.Len("[regimevar]")));
            let mut num8: i32 =  Strings.InStr(str4, ",");
            if (num8 > 0)
            {
              str5: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Left(str4, num8 - 1)))].RegimeSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str4, num8 + 1)))]));
              str1 = Strings.Left(str1, num6 - 1) + str5 + Strings.Mid(str1, num7 + Strings.Len("[/regimevar]"));
              num1 = 1;
            }
          }
        }
      }
      let mut num9: i32 =  1;
      while (num9 == 1)
      {
        num9 = 0;
        let mut num10: i32 =  Strings.InStr(str1, "[regimename]");
        if (num10 > 0)
        {
          let mut num11: i32 =  Strings.InStr(str1, "[/regimename]");
          if (num11 > num10 & num11 > 0)
          {
            name: String = this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num10 + Strings.Len("[regimename]"), num11 - (num10 + Strings.Len("[regimename]")))))].Name;
            str1 = Strings.Left(str1, num10 - 1) + name + Strings.Mid(str1, num11 + Strings.Len("[/regimename]"));
            num9 = 1;
          }
        }
        let mut num12: i32 =  Strings.InStr(str1, "[unitname]");
        if (num12 > 0)
        {
          let mut num13: i32 =  Strings.InStr(str1, "[/unitname]");
          if (num13 > num12 & num13 > 0)
          {
            name: String = this.game.Data.UnitObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num12 + Strings.Len("[unitname]"), num13 - (num12 + Strings.Len("[unitname]")))))].Name;
            str1 = Strings.Left(str1, num12 - 1) + name + Strings.Mid(str1, num13 + Strings.Len("[/unitname]"));
            num9 = 1;
          }
        }
        let mut num14: i32 =  Strings.InStr(str1, "[hexname]");
        if (num14 > 0)
        {
          let mut num15: i32 =  Strings.InStr(str1, "[/hexname]");
          if (num15 > num14 & num15 > 0)
          {
            str6: String = Strings.Mid(str1, num14 + Strings.Len("[hexname]"), num15 - (num14 + Strings.Len("[hexname]")));
            let mut num16: i32 =  Strings.InStr(str6, ",");
            if (num16 > 0)
            {
              hexName: String = this.game.HandyFunctionsObj.GetHexName((int) Math.Round(Conversion.Val(Strings.Left(str6, num16 - 1))), (int) Math.Round(Conversion.Val(Strings.Mid(str6, num16 + 1))), 0);
              str1 = Strings.Left(str1, num14 - 1) + hexName + Strings.Mid(str1, num15 + Strings.Len("[/hexname]"));
              num9 = 1;
            }
          }
        }
      }
      switch (size)
      {
        case 1:
          SizeF sizeF1 = SizeF::new();
          str7: String = Strings.UCase(this.game.Data.ActionCardObj[nr].Title);
          let mut x1: i32 =  (int) Math.Round(4.0 + (88.0 -  Expression.MeasureString(str7, this.game.MarcFont16).Width / 2.0));
          DrawMod.DrawTextColouredMarc(ref Expression, str7, this.game.MarcFont16, x1, 13, c);
          str8: String = Strings.Trim(Conversion.Str( this.game.Data.ActionCardObj[nr].PPCost));
          if (this.game.Data.ActionCardObj[nr].PPCost == -1)
            str8 = "N/A";
          if (this.game.Data.ActionCardObj[nr].PPCost == 0)
          {
            str8 = "FREE";
            if (this.game.Data.ActionCardObj[nr].HisVarCostType > -1)
              str8 = this.game.Data.ActionCardObj[nr].HisVarCostQty.ToString();
          }
          let mut x2: i32 =  (int) Math.Round(72.0 + (19.0 -  Expression.MeasureString(str8, this.game.MarcFont7).Width / 2.0));
          DrawMod.DrawTextColoured(ref Expression, str8, this.game.MarcFont7, x2, 109, Color.White);
          TextAreaClass textAreaClass1 = new TextAreaClass(this.game, 200, 8, this.game.MarcFont8c, "", false, str1, Color.Black, tItemSize: 13, tbackbitmap: (ref bitmap1), bbx: -5, bby: 130, tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local53 = ref Expression;
          bitmap2 = textAreaClass1.Paint();
          ref local54: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local53, ref local54, -5, 130);
          break;
        case 2:
          tText: String = Strings.UCase(str1);
          SizeF sizeF2 = SizeF::new();
          str9: String = Strings.UCase(this.game.Data.ActionCardObj[nr].Title);
          let mut x3: i32 =  (int) Math.Round(3.0 + (46.0 -  Expression.MeasureString(str9, this.game.MarcFont5).Width / 2.0));
          DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont5, x3, 6, c);
          str10: String = Strings.Trim(Conversion.Str( this.game.Data.ActionCardObj[nr].PPCost));
          if (this.game.Data.ActionCardObj[nr].PPCost == -1)
            str10 = "N/A";
          if (this.game.Data.ActionCardObj[nr].PPCost == 0)
          {
            str10 = "FREE";
            if (this.game.Data.ActionCardObj[nr].HisVarCostType > -1)
              str10 = this.game.Data.ActionCardObj[nr].HisVarCostQty.ToString();
          }
          let mut x4: i32 =  (int) Math.Round(39.0 + (11.0 -  Expression.MeasureString(str10, this.game.MarcFont10).Width / 2.0));
          DrawMod.DrawTextColoured(ref Expression, str10, this.game.MarcFont10, x4, 58, Color.White);
          TextAreaClass textAreaClass2 = new TextAreaClass(this.game, 136, 6, this.game.MarcFont11, "", false, tText, Color.Black, tItemSize: 10, tbackbitmap: (ref bitmap1), bbx: -13, bby: 68, tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local55 = ref Expression;
          bitmap2 = textAreaClass2.Paint();
          ref local56: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local55, ref local56, -13, 68);
          if (Shaded)
          {
            DrawMod.DrawBlockGradient2(ref Expression, 0, 0, 100, 145, Color.FromArgb(100, 0, 0, 0), Color.FromArgb(200, 0, 0, 0));
            break;
          }
          break;
        case 3:
          DrawMod.DrawBlock(ref Expression, 3, 25, 25, 1, 0, 0, 0, 195);
          DrawMod.DrawBlock(ref Expression, 3, 29, 18, 1, 0, 0, 0, 185);
          DrawMod.DrawBlock(ref Expression, 3, 3, 16, 1, (int) c.R, (int) c.G, (int) c.B, 200);
          if (Percent > 0)
            DrawMod.DrawTextColoured(ref Expression, Strings.Trim(Conversion.Str( Percent)) + "%", this.game.MarcFont9, 3, 31, Color.Black);
          if (Shaded)
          {
            DrawMod.DrawBlockGradient2(ref Expression, 0, 0, 33, 46, Color.FromArgb(100, 0, 0, 0), Color.FromArgb(200, 0, 0, 0));
            break;
          }
          break;
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return bitmap1;
    }

    pub DrawActionCardMarc2: Bitmap(
      regnr: i32,
      nr: i32,
      let mut roundnr: i32 =  -1,
      let mut size: i32 =  1,
      bool Shaded = false,
      let mut Percent: i32 =  0)
    {
      bitmap1: Bitmap;
      switch (size)
      {
        case 1:
          bitmap1 = new Bitmap(190, 266, PixelFormat.Format32bppPArgb);
          break;
        case 2:
          bitmap1 = new Bitmap(105, 147, PixelFormat.Format32bppPArgb);
          break;
        default:
          bitmap1 = new Bitmap(33, 46, PixelFormat.Format32bppPArgb);
          break;
      }
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) bitmap1);
      c: Color = this.game.Data.ActionCardObj[nr].ColorScheme > 0 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 1 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 2 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 3 ? (this.game.Data.ActionCardObj[nr].ColorScheme != 4 ? Color.White : Color.White) : Color.White) : Color.White) : Color.White) : Color.White;
      bitmap2: Bitmap;
      switch (size)
      {
        case 1:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local1 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1A);
            ref local2: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local1, ref local2, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local3 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2A);
            ref local4: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local3, ref local4, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local5 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3A);
            ref local6: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local5, ref local6, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local7 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4A);
            ref local8: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local7, ref local8, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local9 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5A);
            ref local10: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local9, ref local10, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local11 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6A);
            ref local12: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local11, ref local12, 0, 0);
            break;
          }
          ref Graphics local13 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5A);
          ref local14: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local13, ref local14, 0, 0);
          break;
        case 2:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local15 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1B);
            ref local16: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local15, ref local16, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local17 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2B);
            ref local18: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local17, ref local18, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local19 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3B);
            ref local20: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local19, ref local20, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local21 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4B);
            ref local22: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local21, ref local22, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local23 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5B);
            ref local24: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local23, ref local24, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local25 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6B);
            ref local26: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local25, ref local26, 0, 0);
            break;
          }
          ref Graphics local27 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5B);
          ref local28: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local27, ref local28, 0, 0);
          break;
        default:
          if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
          {
            ref Graphics local29 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD1C);
            ref local30: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local29, ref local30, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
          {
            ref Graphics local31 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD2C);
            ref local32: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local31, ref local32, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
          {
            ref Graphics local33 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD3C);
            ref local34: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local33, ref local34, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
          {
            ref Graphics local35 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD4C);
            ref local36: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local35, ref local36, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
          {
            ref Graphics local37 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5C);
            ref local38: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local37, ref local38, 0, 0);
            break;
          }
          if (this.game.Data.ActionCardObj[nr].ColorScheme == 5)
          {
            ref Graphics local39 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD6C);
            ref local40: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local39, ref local40, 0, 0);
            break;
          }
          ref Graphics local41 = ref Expression;
          bitmap2 = BitmapStore.GetBitmap(this.game.MARCCARD5C);
          ref local42: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local41, ref local42, 0, 0);
          break;
      }
      if (regnr == -1)
        regnr = 0;
      bool flag = false;
      if (this.game.Data.ActionCardObj[nr].EventPicNr > -1 && this.game.Data.Round > 0 && !this.game.Data.RegimeObj[regnr].UseAlternateActionCardPics)
      {
        flag = true;
        switch (size)
        {
          case 1:
            ref Graphics local43 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local44: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local43, ref local44, 8, 37, 165, 76);
            break;
          case 2:
            ref Graphics local45 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local46: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local45, ref local46, 4, 21, 92, 41);
            break;
          default:
            if (this.game.Data.ActionCardObj[nr].SmallGfx > -1)
            {
              ref Graphics local47 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[this.game.Data.ActionCardObj[nr].SmallGfx]);
              ref local48: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local47, ref local48, 1, 6, 29, 14);
              break;
            }
            if (this.game.Data.ActionCardObj[nr].Nato > -1)
            {
              ref Graphics local49 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.ActionCardObj[nr].Nato]);
              ref local50: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local49, ref local50, 1, 6, 29, 14);
              break;
            }
            ref Graphics local51 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local52: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local51, ref local52, 1, 6, 29, 14);
            break;
        }
      }
      if (this.game.Data.Round > 0 && !flag & this.game.Data.ActionCardObj[nr].AlternateEventPicNr > -1 & this.game.Data.RegimeObj[regnr].UseAlternateActionCardPics)
      {
        switch (size)
        {
          case 1:
            ref Graphics local53 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].AlternateEventPicNr]);
            ref local54: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local53, ref local54, 8, 37, 165, 76);
            break;
          case 2:
            ref Graphics local55 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].AlternateEventPicNr]);
            ref local56: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local55, ref local56, 4, 21, 92, 41);
            break;
          default:
            if (this.game.Data.ActionCardObj[nr].SmallGfx > -1)
            {
              ref Graphics local57 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[this.game.Data.ActionCardObj[nr].SmallGfx]);
              ref local58: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local57, ref local58, 1, 6, 29, 14);
              break;
            }
            if (this.game.Data.ActionCardObj[nr].Nato > -1)
            {
              ref Graphics local59 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.ActionCardObj[nr].Nato]);
              ref local60: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local59, ref local60, 1, 6, 29, 14);
              break;
            }
            ref Graphics local61 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].AlternateEventPicNr]);
            ref local62: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local61, ref local62, 1, 6, 29, 14);
            break;
        }
      }
      if (!flag & this.game.Data.ActionCardObj[nr].EventPicNr > -1)
      {
        switch (size)
        {
          case 1:
            ref Graphics local63 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local64: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local63, ref local64, 8, 37, 165, 76);
            break;
          case 2:
            ref Graphics local65 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local66: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local65, ref local66, 4, 21, 92, 41);
            break;
          default:
            if (this.game.Data.ActionCardObj[nr].SmallGfx > -1)
            {
              ref Graphics local67 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[this.game.Data.ActionCardObj[nr].SmallGfx]);
              ref local68: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local67, ref local68, 1, 6, 29, 14);
              break;
            }
            if (this.game.Data.ActionCardObj[nr].Nato > -1)
            {
              ref Graphics local69 = ref Expression;
              bitmap2 = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.ActionCardObj[nr].Nato]);
              ref local70: Bitmap = ref bitmap2;
              DrawMod.DrawScaled(ref local69, ref local70, 1, 6, 29, 14);
              break;
            }
            ref Graphics local71 = ref Expression;
            bitmap2 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local72: Bitmap = ref bitmap2;
            DrawMod.DrawScaled(ref local71, ref local72, 1, 6, 29, 14);
            break;
        }
      }
      str1: String = this.game.Data.ActionCardObj[nr].Text;
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut num2: i32 =  Strings.InStr(str1, "[gamevar]");
        if (num2 > 0)
        {
          let mut num3: i32 =  Strings.InStr(str1, "[/gamevar]");
          if (num3 > num2 & num3 > 0)
          {
            str2: String = Strings.Trim(Conversion.Str( this.game.Data.GameSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num2 + Strings.Len("[gamevar]"), num3 - (num2 + Strings.Len("[gamevar]")))))]));
            str1 = Strings.Left(str1, num2 - 1) + str2 + Strings.Mid(str1, num3 + Strings.Len("[/gamevar]"));
            num1 = 1;
          }
        }
        let mut num4: i32 =  Strings.InStr(str1, "[tempvar]");
        if (num4 > 0)
        {
          let mut num5: i32 =  Strings.InStr(str1, "[/tempvar]");
          if (num5 > num4 & num5 > 0)
          {
            str3: String = Strings.Trim(Conversion.Str( this.game.Data.TempVar[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num4 + Strings.Len("[tempvar]"), num5 - (num4 + Strings.Len("[tempvar]")))))]));
            str1 = Strings.Left(str1, num4 - 1) + str3 + Strings.Mid(str1, num5 + Strings.Len("[/tempvar]"));
            num1 = 1;
          }
        }
        let mut num6: i32 =  Strings.InStr(str1, "[regimevar]");
        if (num6 > 0)
        {
          let mut num7: i32 =  Strings.InStr(str1, "[/regimevar]");
          if (num7 > num6 & num7 > 0)
          {
            str4: String = Strings.Mid(str1, num6 + Strings.Len("[regimevar]"), num7 - (num6 + Strings.Len("[regimevar]")));
            let mut num8: i32 =  Strings.InStr(str4, ",");
            if (num8 > 0)
            {
              str5: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Left(str4, num8 - 1)))].RegimeSlot[(int) Math.Round(Conversion.Val(Strings.Mid(str4, num8 + 1)))]));
              str1 = Strings.Left(str1, num6 - 1) + str5 + Strings.Mid(str1, num7 + Strings.Len("[/regimevar]"));
              num1 = 1;
            }
          }
        }
      }
      let mut num9: i32 =  1;
      while (num9 == 1)
      {
        num9 = 0;
        let mut num10: i32 =  Strings.InStr(str1, "[regimename]");
        if (num10 > 0)
        {
          let mut num11: i32 =  Strings.InStr(str1, "[/regimename]");
          if (num11 > num10 & num11 > 0)
          {
            name: String = this.game.Data.RegimeObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num10 + Strings.Len("[regimename]"), num11 - (num10 + Strings.Len("[regimename]")))))].Name;
            str1 = Strings.Left(str1, num10 - 1) + name + Strings.Mid(str1, num11 + Strings.Len("[/regimename]"));
            num9 = 1;
          }
        }
        let mut num12: i32 =  Strings.InStr(str1, "[unitname]");
        if (num12 > 0)
        {
          let mut num13: i32 =  Strings.InStr(str1, "[/unitname]");
          if (num13 > num12 & num13 > 0)
          {
            name: String = this.game.Data.UnitObj[(int) Math.Round(Conversion.Val(Strings.Mid(str1, num12 + Strings.Len("[unitname]"), num13 - (num12 + Strings.Len("[unitname]")))))].Name;
            str1 = Strings.Left(str1, num12 - 1) + name + Strings.Mid(str1, num13 + Strings.Len("[/unitname]"));
            num9 = 1;
          }
        }
        let mut num14: i32 =  Strings.InStr(str1, "[hexname]");
        if (num14 > 0)
        {
          let mut num15: i32 =  Strings.InStr(str1, "[/hexname]");
          if (num15 > num14 & num15 > 0)
          {
            str6: String = Strings.Mid(str1, num14 + Strings.Len("[hexname]"), num15 - (num14 + Strings.Len("[hexname]")));
            let mut num16: i32 =  Strings.InStr(str6, ",");
            if (num16 > 0)
            {
              hexName: String = this.game.HandyFunctionsObj.GetHexName((int) Math.Round(Conversion.Val(Strings.Left(str6, num16 - 1))), (int) Math.Round(Conversion.Val(Strings.Mid(str6, num16 + 1))), 0);
              str1 = Strings.Left(str1, num14 - 1) + hexName + Strings.Mid(str1, num15 + Strings.Len("[/hexname]"));
              num9 = 1;
            }
          }
        }
      }
      switch (size)
      {
        case 1:
          SizeF sizeF1 = SizeF::new();
          str7: String = Strings.UCase(this.game.Data.ActionCardObj[nr].Title);
          SizeF sizeF2 = Expression.MeasureString(str7, this.game.MarcFont16);
          let mut num17: i32 =  0;
          while ( sizeF2.Width > 160.0)
          {
            sizeF2 = Expression.MeasureString(str7, this.game.MarcFont16);
            str7 = Strings.Left(str7, Strings.Len(str7) - 1);
            num17 = 1;
          }
          if (num17 == 1)
            str7 += "...";
          let mut x1: i32 =  (int) Math.Round(4.0 + (88.0 -  sizeF2.Width / 2.0));
          DrawMod.DrawTextColouredMarc(ref Expression, str7, this.game.MarcFont16, x1, 13, c);
          str8: String = Strings.Trim(Conversion.Str( this.game.Data.ActionCardObj[nr].PPCost));
          if (this.game.Data.ActionCardObj[nr].PPCost == -1)
            str8 = "N/A";
          if (this.game.Data.ActionCardObj[nr].PPCost == 0)
          {
            str8 = "FREE";
            if (this.game.Data.ActionCardObj[nr].HisVarCostType > -1)
              str8 = this.game.Data.ActionCardObj[nr].HisVarCostQty.ToString();
          }
          sizeF2 = Expression.MeasureString(str8, this.game.MarcFont7);
          let mut x2: i32 =  (int) Math.Round(72.0 + (19.0 -  sizeF2.Width / 2.0));
          DrawMod.DrawTextColoured(ref Expression, str8, this.game.MarcFont7, x2, 109, Color.White);
          TextAreaClass textAreaClass1 = new TextAreaClass(this.game, 200, 8, this.game.MarcFont8c, "", false, str1, Color.Black, tItemSize: 13, tbackbitmap: (ref bitmap1), bbx: -5, bby: 130, tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local73 = ref Expression;
          bitmap2 = textAreaClass1.Paint();
          ref local74: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local73, ref local74, -5, 130);
          break;
        case 2:
          tText: String = Strings.UCase(str1);
          SizeF sizeF3 = SizeF::new();
          str9: String = Strings.UCase(this.game.Data.ActionCardObj[nr].Title);
          let mut num18: i32 =  0;
          SizeF sizeF4 = Expression.MeasureString(str9, this.game.MarcFont16);
          while ( sizeF4.Width > 122.0)
          {
            sizeF4 = Expression.MeasureString(str9, this.game.MarcFont16);
            str9 = Strings.Left(str9, Strings.Len(str9) - 1);
            num18 = 1;
          }
          if (num18 == 1)
            str9 += "...";
          let mut x3: i32 =  (int) Math.Round(3.0 + (46.0 -  Expression.MeasureString(str9, this.game.MarcFont5).Width / 2.0));
          DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont5, x3, 6, c);
          str10: String = Strings.Trim(Conversion.Str( this.game.Data.ActionCardObj[nr].PPCost));
          if (this.game.Data.ActionCardObj[nr].PPCost == -1)
            str10 = "N/A";
          if (this.game.Data.ActionCardObj[nr].PPCost == 0)
          {
            str10 = "FREE";
            if (this.game.Data.ActionCardObj[nr].HisVarCostType > -1)
              str10 = this.game.Data.ActionCardObj[nr].HisVarCostQty.ToString();
          }
          let mut x4: i32 =  (int) Math.Round(39.0 + (11.0 -  Expression.MeasureString(str10, this.game.MarcFont10).Width / 2.0));
          DrawMod.DrawTextColoured(ref Expression, str10, this.game.MarcFont10, x4, 58, Color.White);
          TextAreaClass textAreaClass2 = new TextAreaClass(this.game, 136, 6, this.game.MarcFont11, "", false, tText, Color.Black, tItemSize: 10, tbackbitmap: (ref bitmap1), bbx: -13, bby: 68, tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local75 = ref Expression;
          bitmap2 = textAreaClass2.Paint();
          ref local76: Bitmap = ref bitmap2;
          DrawMod.DrawSimple(ref local75, ref local76, -13, 68);
          if (Shaded)
          {
            DrawMod.DrawBlockGradient2(ref Expression, 0, 0, 100, 145, Color.FromArgb(100, 0, 0, 0), Color.FromArgb(200, 0, 0, 0));
            break;
          }
          break;
        case 3:
          DrawMod.DrawBlock(ref Expression, 3, 25, 25, 1, 0, 0, 0, 195);
          DrawMod.DrawBlock(ref Expression, 3, 29, 18, 1, 0, 0, 0, 185);
          DrawMod.DrawBlock(ref Expression, 3, 3, 16, 1, (int) c.R, (int) c.G, (int) c.B, 200);
          if (Percent > 0)
            DrawMod.DrawTextColoured(ref Expression, Strings.Trim(Conversion.Str( Percent)) + "%", this.game.MarcFont9, 3, 31, Color.Black);
          if (Shaded)
          {
            DrawMod.DrawBlockGradient2(ref Expression, 0, 0, 33, 46, Color.FromArgb(100, 0, 0, 0), Color.FromArgb(200, 0, 0, 0));
            break;
          }
          break;
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return bitmap1;
    }

    pub DrawActionCardSe1: Bitmap(
      regnr: i32,
      nr: i32,
      let mut roundnr: i32 =  -1,
      let mut size: i32 =  1,
      bool Shaded = false,
      let mut Percent: i32 =  0,
      let mut tCardId: i32 =  -1)
    {
      let mut stringListById1: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 277, 0, 0));
      bitmap1: Bitmap;
      switch (size)
      {
        case 1:
          bitmap1 = new Bitmap(190, 266, PixelFormat.Format32bppPArgb);
          break;
        case 2:
          bitmap1 = new Bitmap(105, 147, PixelFormat.Format32bppPArgb);
          break;
        default:
          bitmap1 = new Bitmap(33, 46, PixelFormat.Format32bppPArgb);
          break;
      }
      bitmap1.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) bitmap1);
      color: Color = Color.FromArgb((int) byte.MaxValue, 0, (int) byte.MaxValue, 0);
      if (regnr == -1)
        regnr = 0;
      bool flag = false;
      let mut idValue: i32 =  tCardId <= 0 ? this.game.Data.ActionCardObj[nr].TempVar0 : tCardId;
      let mut index1: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 17)));
      let mut index2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 18)));
      let mut num1: i32 =  0;
      let mut index3: i32 =  0;
      let mut index4: i32 =  0;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      if ((int) Math.Round(Conversion.Val(this.game.Data.Designer)) >= 94)
      {
        num1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 20)));
        index3 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 21)));
        index4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 22)));
        num2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 23)));
        num3 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 24)));
      }
      if (tCardId > 0)
      {
        let mut index5: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 3)));
        flag = true;
        switch (size)
        {
          case 1:
            ref Graphics local1 = ref Expression;
            bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index5]);
            ref local2: Bitmap = ref bitmap2;
            DrawMod.DrawSimple(ref local1, ref local2, 0, 0);
            break;
          case 2:
            ref Graphics local3 = ref Expression;
            bitmap3: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index5]);
            ref local4: Bitmap = ref bitmap3;
            DrawMod.DrawScaled(ref local3, ref local4, 0, 0, 105, 147, true);
            break;
        }
      }
      else if (this.game.Data.ActionCardObj[nr].EventPicNr > -1)
      {
        flag = true;
        switch (size)
        {
          case 1:
            ref Graphics local5 = ref Expression;
            bitmap4: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local6: Bitmap = ref bitmap4;
            DrawMod.DrawSimple(ref local5, ref local6, 0, 0);
            break;
          case 2:
            ref Graphics local7 = ref Expression;
            bitmap5: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[this.game.Data.ActionCardObj[nr].EventPicNr]);
            ref local8: Bitmap = ref bitmap5;
            DrawMod.DrawScaled(ref local7, ref local8, 0, 0, 105, 147, true);
            break;
        }
      }
      from: Bitmap;
      if (num1 > 0 && index3 > 0 & index4 > 0)
      {
        bitmap6: Bitmap;
        if (num2 == 1)
        {
          bitmap6 = new Bitmap(190, 266, PixelFormat.Format32bppPArgb);
          bitmap6.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index3]);
          DrawMod.CopyPerLine(ref from, ref bitmap6, 0, 0);
          bitmap7: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index4]);
          DrawMod.CopyPerLineOnlyAlpha(ref bitmap7, ref bitmap6, 0, 0, 190, 266, 0, 0);
          Expression.CompositingMode = CompositingMode.SourceOver;
          Expression.CompositingQuality = CompositingQuality.HighQuality;
          if (num3 < 1 | num3 % 2 == 0)
          {
            switch (size)
            {
              case 1:
                DrawMod.DrawSimple(ref Expression, ref bitmap6, 0, 0);
                break;
              case 2:
                DrawMod.DrawScaled(ref Expression, ref bitmap6, 0, 0, 105, 147, true);
                break;
            }
          }
          else
          {
            let mut num4: i32 =  num3 % (int) byte.MaxValue;
            switch (size)
            {
              case 1:
                DrawMod.Draw(ref Expression, ref bitmap6, 0, 0, 1f, 1f, 1f,  num4 /  byte.MaxValue);
                break;
              case 2:
                DrawMod.DrawScaledColorized(ref Expression, ref bitmap6, 0, 0, 105, 147, 190, 266, 1f, 1f, 1f,  num4 /  byte.MaxValue);
                break;
            }
          }
          bitmap6.Dispose();
        }
        if (num2 == 3)
        {
          bitmap6 = new Bitmap(190, 266, PixelFormat.Format32bppPArgb);
          bitmap6.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index3]);
          DrawMod.CopyPerLine(ref from, ref bitmap6, 0, 0);
          bitmap8: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index4]);
          DrawMod.CopyPerLineOnlyAlpha(ref bitmap8, ref bitmap6, 0, 0, 190, 266, 0, 0);
          Expression.CompositingMode = CompositingMode.SourceOver;
          Expression.CompositingQuality = CompositingQuality.HighQuality;
          let mut maxValue: i32 =  (int) byte.MaxValue;
          switch (size)
          {
            case 1:
              DrawMod.DrawSimple(ref Expression, ref bitmap6, 0, 0);
              break;
            case 2:
              DrawMod.DrawScaledColorized(ref Expression, ref bitmap6, 0, 0, 105, 147, 190, 266, 1f, 1f, 1f,  maxValue /  byte.MaxValue);
              break;
          }
          bitmap6.Dispose();
        }
        if (num2 == 1)
        {
          bitmap6 = this.game.CustomBitmapObj.DrawLeaderPortrait(this.game.EventRelatedObj.Helper_RollCharacter(-1, this.game.Data.Turn, "SE_Data", fixedRandySeed: num3, justForTempGfxUse: true), 100, 140, transBG: true);
          let mut stringListById2: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
          this.game.Data.StringListObj[stringListById2].RemoveRow(this.game.Data.StringListObj[stringListById2].Length);
          switch (size)
          {
            case 1:
              DrawMod.DrawScaled(ref Expression, ref bitmap6, 30, 30, 120, 168, true);
              break;
            case 2:
              DrawMod.DrawScaled(ref Expression, ref bitmap6, 15, 17, 60, 84, true);
              break;
          }
          bitmap6.Dispose();
        }
        if (num2 == 3)
        {
          if (size == 1)
          {
            let mut integer: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 144, 0, 0))].GetData(0, num3, 8));
            if (integer > 0)
            {
              let mut num5: i32 =  0;
              if (BitmapStore.GetWidth(this.game.Data.EventPicNr[integer]) > 190)
                num5 = 1;
              if (BitmapStore.GetWidth(this.game.Data.EventPicNr[integer]) > 360)
                num5 = 2;
              ref Graphics local9 = ref Expression;
              from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[integer]);
              ref local10: Bitmap = ref from;
              Rectangle srcrect = Rectangle::new(num5 * 135, 11, 133, 93);
              Rectangle destrect = Rectangle::new(20, 50, 150, 105);
              DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
            }
          }
          else if (size != 2)
            ;
        }
      }
      if (size == 1)
      {
        if (index1 > 0)
        {
          ref Graphics local11 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index1]);
          ref local12: Bitmap = ref from;
          DrawMod.DrawSimple(ref local11, ref local12, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
        {
          ref Graphics local13 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD1A);
          ref local14: Bitmap = ref from;
          DrawMod.DrawSimple(ref local13, ref local14, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
        {
          ref Graphics local15 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD5A);
          ref local16: Bitmap = ref from;
          DrawMod.DrawSimple(ref local15, ref local16, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
        {
          ref Graphics local17 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD4A);
          ref local18: Bitmap = ref from;
          DrawMod.DrawSimple(ref local17, ref local18, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
        {
          ref Graphics local19 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD6A);
          ref local20: Bitmap = ref from;
          DrawMod.DrawSimple(ref local19, ref local20, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
        {
          ref Graphics local21 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD3A);
          ref local22: Bitmap = ref from;
          DrawMod.DrawSimple(ref local21, ref local22, 0, 0);
        }
      }
      else if (size == 2)
      {
        if (index1 > 0)
        {
          ref Graphics local23 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index2]);
          ref local24: Bitmap = ref from;
          DrawMod.DrawSimple(ref local23, ref local24, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
        {
          ref Graphics local25 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD1B);
          ref local26: Bitmap = ref from;
          DrawMod.DrawSimple(ref local25, ref local26, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
        {
          ref Graphics local27 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD5B);
          ref local28: Bitmap = ref from;
          DrawMod.DrawSimple(ref local27, ref local28, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
        {
          ref Graphics local29 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD4B);
          ref local30: Bitmap = ref from;
          DrawMod.DrawSimple(ref local29, ref local30, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
        {
          ref Graphics local31 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD6B);
          ref local32: Bitmap = ref from;
          DrawMod.DrawSimple(ref local31, ref local32, 0, 0);
        }
        else if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
        {
          ref Graphics local33 = ref Expression;
          from = BitmapStore.GetBitmap(this.game.MARCCARD3B);
          ref local34: Bitmap = ref from;
          DrawMod.DrawSimple(ref local33, ref local34, 0, 0);
        }
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme <= 0)
      {
        ref Graphics local35 = ref Expression;
        from = BitmapStore.GetBitmap(this.game.MARCCARD1C);
        ref local36: Bitmap = ref from;
        DrawMod.DrawSimple(ref local35, ref local36, 0, 0);
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
      {
        ref Graphics local37 = ref Expression;
        from = BitmapStore.GetBitmap(this.game.MARCCARD5C);
        ref local38: Bitmap = ref from;
        DrawMod.DrawSimple(ref local37, ref local38, 0, 0);
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
      {
        ref Graphics local39 = ref Expression;
        from = BitmapStore.GetBitmap(this.game.MARCCARD4C);
        ref local40: Bitmap = ref from;
        DrawMod.DrawSimple(ref local39, ref local40, 0, 0);
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
      {
        ref Graphics local41 = ref Expression;
        from = BitmapStore.GetBitmap(this.game.MARCCARD6C);
        ref local42: Bitmap = ref from;
        DrawMod.DrawSimple(ref local41, ref local42, 0, 0);
      }
      else if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
      {
        ref Graphics local43 = ref Expression;
        from = BitmapStore.GetBitmap(this.game.MARCCARD3C);
        ref local44: Bitmap = ref from;
        DrawMod.DrawSimple(ref local43, ref local44, 0, 0);
      }
      str1: String = tCardId <= 0 ? this.game.Data.ActionCardObj[nr].Text : this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 4);
      c: Color = Color.FromArgb((int) byte.MaxValue, 180, (int) byte.MaxValue, 180);
      tfontcol: Color = Color.FromArgb((int) byte.MaxValue, 225, (int) byte.MaxValue, 225);
      if (tCardId < 1)
      {
        if (this.game.Data.ActionCardObj[nr].ColorScheme == 1)
        {
          c = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 180);
          tfontcol = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 225);
        }
        if (this.game.Data.ActionCardObj[nr].ColorScheme == 2)
        {
          c = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 180, 180);
          tfontcol = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 225, 225);
        }
        if (this.game.Data.ActionCardObj[nr].ColorScheme == 4)
        {
          c = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 240, 225);
          tfontcol = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 250, 245);
        }
        if (this.game.Data.ActionCardObj[nr].ColorScheme == 3)
        {
          c = Color.FromArgb((int) byte.MaxValue, 215, (int) byte.MaxValue, (int) byte.MaxValue);
          tfontcol = Color.FromArgb((int) byte.MaxValue, 240, (int) byte.MaxValue, (int) byte.MaxValue);
        }
      }
      if (index1 > 0)
      {
        c = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
        tfontcol = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
      }
      num6: i32;
      num7: i32;
      switch (size)
      {
        case 1:
          SizeF sizeF1 = SizeF::new();
          str2: String = tCardId <= 0 ? this.game.Data.ActionCardObj[nr].Title : this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 1);
          SizeF sizeF2 = Expression.MeasureString(str2, this.game.se1TypeWriterBig2);
          num6 = 0;
          num7 = 0;
          SizeF sizeF3;
          if ( sizeF2.Width > 170.0)
          {
            strArray: Vec<String> = str2.Split(new char[1]
            {
              ' '
            }, StringSplitOptions.RemoveEmptyEntries);
            str3: String = strArray[0];
            sizeF3 = Expression.MeasureString(str3, this.game.se1TypeWriterBig2);
            num6 = (int) Math.Round(6.0 + (88.0 -  sizeF3.Width / 2.0));
            let mut num8: i32 =  0;
            if (strArray.GetUpperBound(0) > 1)
            {
              str3 = strArray[0] + " " + strArray[1];
              sizeF3 = Expression.MeasureString(str3, this.game.se1TypeWriterBig2);
              num8 = (int) Math.Round(6.0 + (88.0 -  sizeF3.Width / 2.0));
            }
            x1: i32;
            if (num8 > 0 &  sizeF3.Width < 170.0)
            {
              x1 = num8;
            }
            else
            {
              num8 = 0;
              str3 = strArray[0];
              sizeF3 = Expression.MeasureString(str3, this.game.se1TypeWriterBig2);
              x1 = (int) Math.Round(6.0 + (88.0 -  sizeF3.Width / 2.0));
            }
            if (num8 > 0)
              num8 = 1;
            DrawMod.DrawTextColouredConsole(ref Expression, str3, this.game.se1TypeWriterBig2, x1, 0, c);
            str4: String = "";
            let mut num9: i32 =  1 + num8;
            let mut upperBound: i32 =  strArray.GetUpperBound(0);
            for (let mut index6: i32 =  num9; index6 <= upperBound; index6 += 1)
            {
              if (index6 > 1)
                str4 += " ";
              str4 += strArray[index6];
            }
            if (str4.Length > 0)
            {
              num7 = 1;
              sizeF3 = Expression.MeasureString(str4, this.game.se1TypeWriterBig2);
              let mut x2: i32 =  (int) Math.Round(6.0 + (88.0 -  sizeF3.Width / 2.0));
              DrawMod.DrawTextColouredConsole(ref Expression, str4, this.game.se1TypeWriterBig2, x2, 13, c);
            }
          }
          else
          {
            let mut num10: i32 =  (int) Math.Round( sizeF2.Width);
            sizeF2 = Expression.MeasureString(str2, this.game.se1TypeWriterBig);
            if ( sizeF2.Width < 170.0)
            {
              let mut x: i32 =  (int) Math.Round(6.0 + (88.0 -  sizeF2.Width / 2.0));
              DrawMod.DrawTextColouredConsole(ref Expression, str2, this.game.se1TypeWriterBig, x, 4, c);
            }
            else
            {
              let mut x: i32 =  (int) Math.Round(6.0 + (88.0 -  num10 / 2.0));
              DrawMod.DrawTextColouredConsole(ref Expression, str2, this.game.se1TypeWriterBig2, x, 6, c);
            }
          }
          if (tCardId < 1)
          {
            str5: String = this.game.Data.ActionCardObj[nr].PPCost.ToString();
            if (this.game.Data.ActionCardObj[nr].PPCost == -1)
              str5 = "N/A";
            if (this.game.Data.ActionCardObj[nr].PPCost == 0)
            {
              str5 = "FREE";
              if (this.game.Data.ActionCardObj[nr].customCostType > -1 & this.game.Data.ActionCardObj[nr].customCostQty > 0)
                str5 = this.game.Data.ActionCardObj[nr].customCostQty.ToString();
            }
            sizeF3 = Expression.MeasureString(str5, this.game.shadowFontConsole2);
            let mut x3: i32 =  (int) Math.Round(122.0 + (29.0 - ( sizeF3.Width + 16.0) / 2.0) + 8.0);
            if (this.game.Data.ActionCardObj[nr].PPCost > 0 | this.game.Data.ActionCardObj[nr].customCostQty > 0)
              x3 -= 8;
            DrawMod.DrawTextColouredConsole(ref Expression, str5, this.game.shadowFontConsole2, x3, 160, c);
            let mut num11: i32 =  (int) Math.Round( x3 +  sizeF3.Width - 8.0);
            if (this.game.Data.ActionCardObj[nr].PPCost > 0 | this.game.Data.ActionCardObj[nr].customCostQty > 0)
            {
              if (this.game.Data.ActionCardObj[nr].customCostType == 1)
              {
                let mut eventPicSlotFor: i32 =  this.game.EventRelatedObj.GetEventPicSlotFor(0, "", "fp");
                ref Graphics local45 = ref Expression;
                from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[eventPicSlotFor]);
                ref local46: Bitmap = ref from;
                let mut x4: i32 =  num11;
                DrawMod.DrawSimple(ref local45, ref local46, x4, 164);
              }
              else
              {
                let mut eventPicSlotFor: i32 =  this.game.EventRelatedObj.GetEventPicSlotFor(0, "", "pp");
                ref Graphics local47 = ref Expression;
                from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[eventPicSlotFor]);
                ref local48: Bitmap = ref from;
                let mut x5: i32 =  num11;
                DrawMod.DrawSimple(ref local47, ref local48, x5, 164);
              }
            }
          }
          tText1: String = str1;
          let mut num12: i32 =  (int) Math.Round( (75 - new TextAreaClass(this.game, 230, 4, this.game.se1TypeWriterSmall, "", false, tText1, tfontcol, tItemSize: 13, tbackbitmap: (ref bitmap1), bbx: -20, bby: 189, tcenterit: true, tHideShade: true, tBlockScroller: true).HeightUsed()) / 2.0);
          TextAreaClass textAreaClass1 = new TextAreaClass(this.game, 230, 4, this.game.se1TypeWriterSmall, "", false, tText1, tfontcol, tItemSize: 13, tbackbitmap: (ref bitmap1), bbx: -16, bby: (179 + num12), tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local49 = ref Expression;
          from = textAreaClass1.Paint();
          ref local50: Bitmap = ref from;
          let mut y1: i32 =  179 + num12;
          DrawMod.DrawSimple(ref local49, ref local50, -16, y1);
          break;
        case 2:
          SizeF sizeF4 = SizeF::new();
          str6: String = tCardId <= 0 ? this.game.Data.ActionCardObj[nr].Title : this.game.Data.StringListObj[stringListById1].GetData(0, idValue, 1);
          num6 = 0;
          SizeF sizeF5 = Expression.MeasureString(str6, this.game.se1TypeWriterBig3);
          num7 = 0;
          SizeF sizeF6;
          if ( sizeF5.Width > 100.0)
          {
            strArray: Vec<String> = str6.Split(new char[1]
            {
              ' '
            }, StringSplitOptions.RemoveEmptyEntries);
            str7: String = strArray[0];
            sizeF6 = Expression.MeasureString(str7, this.game.se1TypeWriterBig3);
            num6 = (int) Math.Round(6.0 + (44.0 -  sizeF6.Width / 2.0));
            let mut num13: i32 =  0;
            if (strArray.GetUpperBound(0) > 1)
            {
              str7 = strArray[0] + " " + strArray[1];
              sizeF6 = Expression.MeasureString(str7, this.game.se1TypeWriterBig3);
              num13 = (int) Math.Round(6.0 + (88.0 -  sizeF6.Width / 2.0));
            }
            if (num13 > 0 &  sizeF6.Width < 80.0)
            {
              num6 = num13;
            }
            else
            {
              num13 = 0;
              str7 = strArray[0];
              sizeF6 = Expression.MeasureString(str7, this.game.se1TypeWriterBig3);
              num6 = (int) Math.Round(6.0 + (44.0 -  sizeF6.Width / 2.0));
            }
            if (num13 > 0)
              num13 = 1;
            let mut x6: i32 =  (int) Math.Round(6.0 + (46.0 -  sizeF6.Width / 2.0));
            DrawMod.DrawTextColouredConsole(ref Expression, str7, this.game.se1TypeWriterBig3, x6, -2, c);
            str8: String = "";
            let mut num14: i32 =  1 + num13;
            let mut upperBound: i32 =  strArray.GetUpperBound(0);
            for (let mut index7: i32 =  num14; index7 <= upperBound; index7 += 1)
            {
              if (index7 > 1)
                str8 += " ";
              str8 += strArray[index7];
            }
            if (str8.Length > 0)
            {
              num7 = 1;
              sizeF6 = Expression.MeasureString(str8, this.game.se1TypeWriterBig3);
              let mut x7: i32 =  (int) Math.Round(6.0 + (46.0 -  sizeF6.Width / 2.0));
              DrawMod.DrawTextColouredConsole(ref Expression, str8, this.game.se1TypeWriterBig3, x7, 8, c);
            }
          }
          else
          {
            let mut x: i32 =  (int) Math.Round(6.0 + (46.0 -  sizeF5.Width / 2.0));
            DrawMod.DrawTextColouredConsole(ref Expression, str6, this.game.se1TypeWriterBig3, x, 3, c);
          }
          eventPicSlotFor1: i32;
          if (tCardId < 1)
          {
            str9: String = this.game.Data.ActionCardObj[nr].PPCost.ToString();
            if (this.game.Data.ActionCardObj[nr].PPCost == -1)
              str9 = "N/A";
            if (this.game.Data.ActionCardObj[nr].PPCost == 0)
            {
              str9 = "FREE";
              if (this.game.Data.ActionCardObj[nr].customCostType > -1 & this.game.Data.ActionCardObj[nr].customCostQty > 0)
                str9 = this.game.Data.ActionCardObj[nr].customCostQty.ToString();
            }
            sizeF6 = Expression.MeasureString(str9, this.game.shadowFontConsole);
            let mut x8: i32 =  (int) Math.Round(58.0 + (25.0 -  sizeF6.Width / 2.0) - 3.0);
            if (this.game.Data.ActionCardObj[nr].PPCost > 0 | this.game.Data.ActionCardObj[nr].customCostQty > 0)
              x8 -= 5;
            DrawMod.DrawTextColouredConsole(ref Expression, str9, this.game.shadowFontConsole, x8, 87, c);
            let mut num15: i32 =  (int) Math.Round( x8 +  sizeF6.Width - 10.0);
            if (this.game.Data.ActionCardObj[nr].PPCost > 0 | this.game.Data.ActionCardObj[nr].customCostQty > 0)
            {
              if (this.game.Data.ActionCardObj[nr].customCostType == 1)
              {
                eventPicSlotFor1 = this.game.EventRelatedObj.GetEventPicSlotFor(0, "", "fp");
                ref Graphics local51 = ref Expression;
                from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[eventPicSlotFor1]);
                ref local52: Bitmap = ref from;
                let mut x9: i32 =  num15 + 1;
                DrawMod.DrawSimple(ref local51, ref local52, x9, 85);
              }
              else
              {
                eventPicSlotFor1 = this.game.EventRelatedObj.GetEventPicSlotFor(0, "", "pp");
                ref Graphics local53 = ref Expression;
                from = BitmapStore.GetBitmap(this.game.Data.EventPicNr[eventPicSlotFor1]);
                ref local54: Bitmap = ref from;
                let mut x10: i32 =  num15 + 1;
                DrawMod.DrawSimple(ref local53, ref local54, x10, 85);
              }
            }
          }
          tText2: String = str1;
          let mut num16: i32 =  (int) Math.Round(Math.Max(0.0,  (27 - new TextAreaClass(this.game, 150, 3, this.game.shadowFontConsole3, "", false, tText2, tfontcol, tItemSize: 8, tbackbitmap: (ref bitmap1), bbx: -18, bby: (103 + eventPicSlotFor1), tcenterit: true, tHideShade: true, tBlockScroller: true).HeightUsed()) / 2.0));
          TextAreaClass textAreaClass2 = new TextAreaClass(this.game, 150, 3, this.game.shadowFontConsole3, "", false, tText2, tfontcol, tItemSize: 8, tbackbitmap: (ref bitmap1), bbx: -18, bby: (103 + num16), tcenterit: true, tHideShade: true, tBlockScroller: true);
          ref Graphics local55 = ref Expression;
          from = textAreaClass2.Paint();
          ref local56: Bitmap = ref from;
          let mut y2: i32 =  103 + num16;
          DrawMod.DrawSimple(ref local55, ref local56, -18, y2);
          if (!Shaded)
            break;
          break;
        case 3:
          DrawMod.DrawBlock(ref Expression, 3, 25, 25, 1, 0, 0, 0, 195);
          DrawMod.DrawBlock(ref Expression, 3, 29, 18, 1, 0, 0, 0, 185);
          DrawMod.DrawBlock(ref Expression, 3, 3, 16, 1, (int) color.R, (int) color.G, (int) color.B, 200);
          break;
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return bitmap1;
    }

    pub fn InitializeTextureRelatedStuff()
    {
      if ( this.game.Data.RuleVar[451] > 0.0 & this.game.Data.Product >= 7)
      {
        let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[451]));
        let mut length: i32 =  this.game.Data.StringListObj[stringListById].Length;
        for (let mut index1: i32 =  0; index1 <= length; index1 += 1)
        {
          if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 0])) > 0)
          {
            str: String = this.game.Data.StringListObj[stringListById].Data[index1, 1];
            let mut fr: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 2]));
            let mut num: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 3]));
            let mut fb: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 4]));
            let mut tr: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 5]));
            let mut tg: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 6]));
            let mut tb: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 7]));
            bool flag = false;
            if (Strings.InStr(str, "Forests") > 0)
              str = str;
            if (Operators.CompareString(Strings.Trim(Strings.LCase(this.game.Data.StringListObj[stringListById].Data[index1, 2])), "reload", false) == 0)
              BitmapStore.ReloadBeforeRecolor(str, this.game.Data.StringListObj[stringListById].Data[index1, 3]);
            else
              flag = Operators.CompareString(Strings.Trim(Strings.LCase(this.game.Data.StringListObj[stringListById].Data[index1, 2])), "gray", false) == 0 ? BitmapStore.ModifyColorOfNameInstrToGray(str, num) : BitmapStore.ModifyColorOfNameInstr(str, fr, num, fb, tr, tg, tb);
            if ( this.game.Data.RuleVar[998] == 1.0 & flag)
            {
              let mut landscapeTypeCounter: i32 =  this.game.Data.LandscapeTypeCounter;
              for (let mut index2: i32 =  0; index2 <= landscapeTypeCounter; index2 += 1)
              {
                this.game.Data.LandscapeTypeObj[index2].TempHexBitmap = (Bitmap) null;
                if (this.game.Data.LandscapeTypeObj[index2].UsePreHexTexture && Strings.InStr(Strings.LCase(this.game.Data.LandscapeTypeObj[index2].PreHexTextureFileName).Replace("\\", "/").Replace("//", "/"), Strings.Trim(Strings.LCase(str))) > 0)
                {
                  let mut preHexTextureId: i32 =  this.game.Data.LandscapeTypeObj[index2].PreHexTextureID;
                  if (preHexTextureId > -1 & BitmapStore.simpleByteCacheSet[preHexTextureId])
                    BitmapStore.simpleByteCacheSet[preHexTextureId] = false;
                }
              }
            }
          }
        }
      }
      if ( this.game.Data.RuleVar[998] == 1.0)
      {
        try
        {
          DrawMod.TGame.FormRef.Cursor = Cursors.WaitCursor;
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        if (Information.IsNothing( this.game.CustomBitmapObj.tempHexSmall))
        {
          this.game.CustomBitmapObj.tempHexSmall = new Bitmap(32, 24, PixelFormat.Format32bppPArgb);
          this.game.CustomBitmapObj.tempHexSmall.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          this.game.CustomBitmapObj.tempHexMed = new Bitmap(64, 48, PixelFormat.Format32bppPArgb);
          this.game.CustomBitmapObj.tempHexMed.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          this.game.CustomBitmapObj.tempHexBig = new Bitmap(128, 96, PixelFormat.Format32bppPArgb);
          this.game.CustomBitmapObj.tempHexBig.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        }
      }
      if ( this.game.Data.RuleVar[998] != 1.0)
        return;
      let mut landscapeTypeCounter1: i32 =  this.game.Data.LandscapeTypeCounter;
      bitmap1: Bitmap;
      for (let mut index: i32 =  0; index <= landscapeTypeCounter1; index += 1)
      {
        if (this.game.Data.LandscapeTypeObj[index].UsePreHexTexture | this.game.Data.LandscapeTypeObj[index].UsePreHexBorderTexture)
        {
          let mut preHexTextureId: i32 =  this.game.Data.LandscapeTypeObj[index].PreHexTextureID;
          let mut sheetSpriteId1: i32 =  this.game.Data.LandscapeTypeObj[index].SheetSpriteID;
          if (!BitmapStore.simpleByteCacheSet[preHexTextureId])
          {
            if (this.game.Data.LandscapeTypeObj[index].UsePreHexTexture)
            {
              BitmapStore.simpleByteCacheObj[preHexTextureId] = SimpleByteCache::new();
              SimpleByteCache simpleByteCache = BitmapStore.simpleByteCacheObj[preHexTextureId];
              bitmap2: Bitmap = BitmapStore.GetBitmap(preHexTextureId);
              ref local1: Bitmap = ref bitmap2;
              bitmap3: Bitmap = BitmapStore.GetBitmap(preHexTextureId, 1);
              ref local2: Bitmap = ref bitmap3;
              bitmap1 = BitmapStore.GetBitmap(preHexTextureId, -1);
              ref local3: Bitmap = ref bitmap1;
              let mut bitmapNr: i32 =  preHexTextureId;
              simpleByteCache.SetMultiRGBCache(ref local1, ref local2, ref local3, bitmapNr);
            }
            BitmapStore.simpleByteCacheSet[preHexTextureId] = true;
          }
          if (!BitmapStore.simpleByteCacheSet[sheetSpriteId1])
          {
            if (this.game.Data.LandscapeTypeObj[index].UseSheet & this.game.Data.LandscapeTypeObj[index].UsePreHexBorderTexture)
            {
              BitmapStore.simpleByteCacheObj[sheetSpriteId1] = SimpleByteCache::new();
              SimpleByteCache simpleByteCache = BitmapStore.simpleByteCacheObj[sheetSpriteId1];
              bitmap1 = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index].SheetSpriteID);
              ref local4: Bitmap = ref bitmap1;
              bitmap4: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index].SheetSpriteID, 1);
              ref local5: Bitmap = ref bitmap4;
              bitmap5: Bitmap = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index].SheetSpriteID, -1);
              ref local6: Bitmap = ref bitmap5;
              let mut sheetSpriteId2: i32 =  this.game.Data.LandscapeTypeObj[index].SheetSpriteID;
              simpleByteCache.SetSingleFredAlphaCache(ref local4, ref local5, ref local6, sheetSpriteId2);
            }
            BitmapStore.simpleByteCacheSet[sheetSpriteId1] = true;
          }
        }
      }
      if (!BitmapStore.simpleByteCacheSet[this.game.WHITEHEX])
      {
        BitmapStore.simpleByteCacheObj[this.game.WHITEHEX] = SimpleByteCache::new();
        SimpleByteCache simpleByteCache = BitmapStore.simpleByteCacheObj[this.game.WHITEHEX];
        bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEX);
        ref local7: Bitmap = ref bitmap1;
        bitmap6: Bitmap = BitmapStore.GetBitmap(this.game.WHITEHEX, 1);
        ref local8: Bitmap = ref bitmap6;
        bitmap7: Bitmap = BitmapStore.GetBitmap(this.game.WHITEHEX, -1);
        ref local9: Bitmap = ref bitmap7;
        simpleByteCache.SetSingleAlphaCache(ref local7, ref local8, ref local9);
        BitmapStore.simpleByteCacheSet[this.game.WHITEHEX] = true;
      }
      try
      {
        DrawMod.TGame.FormRef.Cursor = Cursors.Default;
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
    }

    pub void MakeMiniMap(
      tempBmp: Bitmap,
      bwidth: i32,
      bheight: i32,
      bool alsounits,
      bool realhex = false,
      bool alsoshading = true,
      bool predrawn = false,
      let mut humanplayer: i32 =  -1,
      bool showflag = false,
      bool alsoHQ = false,
      let mut highlightTempvar4: i32 =  -1,
      bool useTempVar3asAlpha = false,
      bool useTempAi2 = false,
      bool useTempZones = false,
      let mut specialMode1: i32 =  -1)
    {
      SizeF sizeF = SizeF::new();
      bool[] flagArray = new bool[this.game.Data.RegimeCounter + 99 + 1];
      int[] numArray1 = new int[this.game.Data.RegimeCounter + 99 + 1];
      int[] numArray2 = new int[this.game.Data.RegimeCounter + 99 + 1];
      let mut stringListById1: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0));
      let mut stringListById2: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 288, 0, 0));
      let mut stringListById3: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
      let mut stringListById4: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Random", 86, 0, 0));
      let mut stringListById5: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0));
      let mut num1: i32 =  0;
      if (stringListById4 > 0)
        num1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById4].GetData(0, 84, 2)));
      libVar1: i32;
      libVar2: i32;
      libVar3: i32;
      libVar4: i32;
      libVar5: i32;
      libVar6: i32;
      libVar7: i32;
      str1: String;
      libVar8: i32;
      if (specialMode1 > 0)
      {
        data1: DataClass = this.game.Data;
        str2: String = "MiningType";
        ref local1: String = ref str2;
        libVar1 = data1.FindLibVar(ref local1, "SE_Data");
        data2: DataClass = this.game.Data;
        str3: String = "MiningDiscovery";
        ref local2: String = ref str3;
        libVar2 = data2.FindLibVar(ref local2, "SE_Data");
        data3: DataClass = this.game.Data;
        str4: String = "MiningReserve";
        ref local3: String = ref str4;
        libVar3 = data3.FindLibVar(ref local3, "SE_Data");
        data4: DataClass = this.game.Data;
        str4 = "Rain";
        ref local4: String = ref str4;
        libVar4 = data4.FindLibVar(ref local4, "SE_Data");
        data5: DataClass = this.game.Data;
        str4 = "Temperature";
        ref local5: String = ref str4;
        libVar5 = data5.FindLibVar(ref local5, "SE_Data");
        data6: DataClass = this.game.Data;
        str5: String = "rad";
        ref local6: String = ref str5;
        libVar6 = data6.FindLibVar(ref local6, "SE_Data");
        data7: DataClass = this.game.Data;
        str6: String = "HeightMap";
        ref local7: String = ref str6;
        data7.FindLibVar(ref local7, "SE_Random");
        data8: DataClass = this.game.Data;
        str6 = "TectonicPlates";
        ref local8: String = ref str6;
        libVar7 = data8.FindLibVar(ref local8, "SE_Random");
        data9: DataClass = this.game.Data;
        str1 = "Scavenge";
        ref local9: String = ref str1;
        libVar8 = data9.FindLibVar(ref local9, "SE_Data");
      }
      flagArray[0] = true;
      flagArray[1] = true;
      let mut Index: i32 =  this.game.Data.Turn;
      if (this.game.EditObj.AIMoving && this.game.EditObj.HumanPlayer > -1)
        Index = this.game.EditObj.HumanPlayer;
      let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
      for (let mut index: i32 =  2; index <= regimeCounter1; index += 1)
      {
        let mut num2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, this.game.Data.RegimeObj[index].id, 1)));
        if ((num2 == 2 | num2 == 3) & this.game.Data.Round > 0 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData3(0, this.game.Data.RegimeObj[index].id, 1, this.game.Data.RegimeObj[Index].id, 2, "dipClear", 3))) < 1)
          flagArray[index] = true;
        if (Index > -1)
        {
          let mut num3: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData3(0, this.game.Data.RegimeObj[index].id, 1, this.game.Data.RegimeObj[Index].id, 2, "relation", 3)));
          numArray1[index] = num3;
          let mut num4: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(0, this.game.Data.RegimeObj[index].id, 1)));
          numArray2[index] = num4;
        }
      }
      data: DataClass = this.game.Data;
      str1 = "zones";
      ref local10: String = ref str1;
      let mut libVar9: i32 =  data.FindLibVar(ref local10, "SE_Data");
      DrawMod.TGame.CustomBitmapObj.InitializeTextureRelatedStuff();
      Graphics Expression = Graphics.FromImage((Image) tempBmp);
      if (!predrawn | Information.IsNothing( this.miniMapPredrawnCache) && !predrawn)
      {
        DrawMod.Clear(ref Expression, 0, 0, 0);
        DrawMod.Clear(ref Expression, 60, 60, 60);
        Pen pen = new Pen(Color.FromArgb((int) byte.MaxValue, 80, 80, 80));
        let mut num5: i32 =  -this.game.ScreenHeight + this.game.ScreenHeight % 6;
        let mut screenHeight: i32 =  this.game.ScreenHeight;
        for (let mut index: i32 =  3; index <= screenHeight; index += 6)
        {
          let mut x1: i32 =  0;
          let mut y1: i32 =  index;
          let mut screenWidth: i32 =  this.game.ScreenWidth;
          let mut y2: i32 =  index;
          Expression.DrawLine(pen, x1, y1, screenWidth, y2);
        }
      }
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth == -1 | this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight == -1)
        return;
      float d1 =  bwidth /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1);
      float d2 =  bheight /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1);
      if (bwidth > 310)
      {
        d1 =  Math.Floor( d1);
        d2 =  Math.Floor( d2);
      }
      num6: i32;
      if ( d1 >  d2)
      {
        num6 = (int) Math.Round( bwidth / 2.0 -  d2 /  d1 * ( bwidth / 2.0));
        d1 = d2;
      }
      num7: i32;
      if ( d2 >  d1)
      {
        num7 = (int) Math.Round( bheight / 2.0 -  d1 /  d2 * ( bheight / 2.0));
        d2 = d1;
      }
      if (bwidth > 310)
      {
        let mut num8: i32 =  (int) Math.Round( ( bwidth -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1) * d1));
        if (num8 > 0)
        {
          let mut num9: i32 =  (int) Math.Round( num8 / 2.0);
          if (num9 > num6)
            num6 = num9;
        }
      }
      if (bheight > 220)
      {
        let mut num10: i32 =  (int) Math.Round( ( bheight -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1) * d2));
        if (num10 > 0)
        {
          let mut num11: i32 =  (int) Math.Round( num10 / 2.0);
          if (num11 > num7)
            num7 = num11;
        }
      }
      float num12 = d1;
      float num13 = d2;
      let mut index1: i32 =  Index;
      if (humanplayer > -1 & this.game.EditObj.AIMoving)
        index1 = humanplayer;
      bool flag1;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (realhex & Information.IsNothing( this.miniMapPredrawnCache))
      {
        this.miniMapPredrawnCache = (Bitmap) tempBmp.Clone();
        Graphics graphics = Graphics.FromImage((Image) this.miniMapPredrawnCache);
        let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            let mut index4: i32 =  index2;
            let mut index5: i32 =  index3;
            if (this.game.EditObj.MiniMapOffset > 0)
            {
              index4 += this.game.EditObj.MiniMapOffset;
              if (index4 > this.game.Data.MapObj[0].MapWidth)
                index4 -= this.game.Data.MapObj[0].MapWidth + 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index4, index5].LandscapeType > -1)
            {
              let mut x: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index2 +  num6)));
              let mut y: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index3) - Math.Floor( d2 / 2.0) +  num7));
              if ((index2 + 10) % 2 > 0)
                y = (int) Math.Round( y + Math.Floor( d2 / 2.0));
              let mut width: i32 =  (int) Math.Round(Math.Floor( num12));
              let mut height: i32 =  (int) Math.Round(Math.Floor( num13));
              flag1 = false;
              let mut num14: i32 =  (int) Math.Round( this.game.Data.RuleVar[330]);
              this.game.Data.RuleVar[330] = 0.0f;
              bool fowOn = this.game.Data.FOWOn;
              this.game.Data.FOWOn = false;
              ref Graphics local11 = ref graphics;
              let mut cx: i32 =  index4;
              let mut cy: i32 =  index5;
              let mut mapSelected: i32 =  this.game.EditObj.MapSelected;
              bitmap1: Bitmap = (Bitmap) null;
              ref local12: Bitmap = ref bitmap1;
              bool flag2 = false;
              ref bool local13 = ref flag2;
              bitmap2: Bitmap = this.DrawHex(cx, cy, mapSelected, true, true, true, Zoom: -1, gBitmap: (ref local12), tFromMapPopup: (ref local13));
              ref local14: Bitmap = ref bitmap2;
              rectangle1 = Rectangle::new(6, 0, 20, 24);
              let mut srcrect: &Rectangle = &rectangle1
              rectangle2 = Rectangle::new(x, y, width, height);
              let mut destrect: &Rectangle = &rectangle2
              DrawMod.DrawSimplePart2(ref local11, ref local14, srcrect, destrect);
              this.game.Data.FOWOn = fowOn;
              this.game.Data.RuleVar[330] =  num14;
            }
          }
        }
        graphics.Dispose();
        Expression.DrawImageUnscaled((Image) this.miniMapPredrawnCache, 0, 0);
      }
      else if (!predrawn & !Information.IsNothing( this.miniMapPredrawnCache) & realhex)
        Expression.DrawImageUnscaled((Image) this.miniMapPredrawnCache, 0, 0);
      let mut mapWidth1: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
      bool flag3;
      bitmap: Bitmap;
      Coordinate coordinate;
      for (let mut index6: i32 =  0; index6 <= mapWidth1; index6 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
        for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
        {
          let mut cx: i32 =  index6;
          let mut cy: i32 =  index7;
          if (this.game.EditObj.MiniMapOffset > 0)
          {
            cx += this.game.EditObj.MiniMapOffset;
            if (cx > this.game.Data.MapObj[0].MapWidth)
              cx -= this.game.Data.MapObj[0].MapWidth + 1;
          }
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType > -1)
          {
            num15: i32;
            num16: i32;
            num17: i32;
            num18: i32;
            if (!realhex)
            {
              num15 = (int) Math.Round( (Conversion.Int(num12 *  index6) +  num6));
              num16 = (int) Math.Round( (Conversion.Int(num13 *  index7) - d2 / 2f +  num7));
              if ((index6 + 10) % 2 > 0)
                num16 = (int) Math.Round( ( num16 + d2 / 2f));
              num17 = (int) Math.Round( (num12 + 1f));
              num18 = (int) Math.Round( (num13 + 1f));
            }
            else
            {
              num15 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index6 +  num6)));
              num16 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index7) - Math.Floor( d2 / 2.0) +  num7));
              let mut num19: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  (index7 + 1)) - Math.Floor( d2 / 2.0) +  num7));
              if ((index6 + 10) % 2 > 0)
                num16 = (int) Math.Round( num16 + Math.Floor( d2 / 2.0));
              if ((index6 + 10) % 2 > 0)
                num19 = (int) Math.Round( num19 + Math.Floor( d2 / 2.0));
              num17 = (int) Math.Round(Math.Floor( num12));
              num18 = num19 - num16;
              if ( num18 != Math.Floor( num13))
                num18 = num18;
            }
            bool flag4 = false;
            bool flag5 = false;
            color: Color;
            index8: i32;
            if (!this.game.Data.ShrowdOn)
            {
              if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].Red == -1)
              {
                if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].IsSea)
                {
                  color = Color.FromArgb((int) byte.MaxValue, 0, 0, 150);
                }
                else
                {
                  index8 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime;
                  if (index8 == -1)
                  {
                    color = Color.FromArgb((int) byte.MaxValue, 40, 200, 60);
                  }
                  else
                  {
                    if (flagArray[index8])
                      index8 = 1;
                    color = Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[index8].Red, this.game.Data.RegimeObj[index8].Green, this.game.Data.RegimeObj[index8].Blue);
                  }
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Location > -1 & num17 >= 4 & num18 >= 4 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Location].Type].MaxProd > 0)
                    color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].VP > 0 & num17 >= 4 & num18 >= 4)
                    color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                }
              }
              else
              {
                color = Color.FromArgb((int) byte.MaxValue, this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].Red, this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].Green, this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].Blue);
                index8 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime;
                if (index8 > -1 & !predrawn)
                {
                  if (flagArray[index8])
                    index8 = 1;
                  color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( color.R * 0.8 + 0.2 *  this.game.Data.RegimeObj[index8].Red), (int) Math.Round( color.G * 0.8 + 0.2 *  this.game.Data.RegimeObj[index8].Green), (int) Math.Round( color.B * 0.8 + 0.2 *  this.game.Data.RegimeObj[index8].Blue));
                }
              }
            }
            if (this.game.Data.ShrowdOn)
              flag3 = true;
            bool flag6 = false;
            if (this.game.Data.ShrowdOn & this.game.Data.Round != 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn))
              {
                flag5 = true;
                flag3 = false;
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastLT(index1) == -1)
                {
                  color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                  flag4 = true;
                  flag3 = true;
                }
                else if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].IsSea)
                {
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastReg(index1) > -1)
                  {
                    index8 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastReg(index1);
                    color = Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[index8].Red, this.game.Data.RegimeObj[index8].Green, this.game.Data.RegimeObj[index8].Blue);
                  }
                  else
                    color = Color.FromArgb((int) byte.MaxValue, 40, 200, 60);
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Location].Type].MaxProd > 0)
                    color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].VP > 0)
                    color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                }
                else
                {
                  let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType;
                  color = Color.FromArgb((int) byte.MaxValue, this.game.Data.LandscapeTypeObj[landscapeType].Red, this.game.Data.LandscapeTypeObj[landscapeType].Green, this.game.Data.LandscapeTypeObj[landscapeType].Blue);
                }
              }
              else
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastLT(index1) > -1)
                {
                  if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].LandscapeType].IsSea)
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastReg(index1) > -1)
                    {
                      flag5 = true;
                      index8 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastReg(index1);
                      if (index8 <= this.game.Data.RegimeCounter)
                      {
                        color = Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[index8].Red, this.game.Data.RegimeObj[index8].Green, this.game.Data.RegimeObj[index8].Blue);
                      }
                      else
                      {
                        color = Color.FromArgb((int) byte.MaxValue, 200, 200, 200);
                        flag6 = true;
                      }
                    }
                    else
                    {
                      color = Color.FromArgb((int) byte.MaxValue, 200, 200, 200);
                      flag6 = true;
                    }
                  }
                  else
                  {
                    let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType;
                    color = Color.FromArgb((int) byte.MaxValue, this.game.Data.LandscapeTypeObj[landscapeType].Red, this.game.Data.LandscapeTypeObj[landscapeType].Green, (int) byte.MaxValue);
                    flag6 = true;
                  }
                }
                else
                {
                  color = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
                  flag6 = true;
                  flag4 = true;
                }
                if (flag6 & useTempVar3asAlpha && this.game.EditObj.TempValue3[0].Value[cx, cy] > 0)
                {
                  if (this.game.EditObj.TempAI[cx, cy] > -1)
                  {
                    index8 = this.game.EditObj.TempAI[cx, cy];
                    if (index8 > -1)
                    {
                      flag5 = true;
                      color = Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[index8].Red, this.game.Data.RegimeObj[index8].Green, this.game.Data.RegimeObj[index8].Blue);
                    }
                    else
                      color = Color.White;
                  }
                  else
                    color = Color.White;
                }
              }
            }
            if (index1 > -1 & !realhex)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastReg(index1) == index1 | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime == index1)
              {
                let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType;
                color = Color.FromArgb((int) byte.MaxValue, this.game.Data.LandscapeTypeObj[landscapeType].Red, this.game.Data.LandscapeTypeObj[landscapeType].Green, this.game.Data.LandscapeTypeObj[landscapeType].Blue);
              }
              else if (useTempAi2)
              {
                let mut index9: i32 =  this.game.EditObj.TempAI2[cx, cy];
                if (index9 > -1)
                  color = Color.FromArgb((int) byte.MaxValue, Math.Min((int) byte.MaxValue, (int) Math.Round( this.game.Data.LandscapeTypeObj[index9].Red * 0.5 +  color.R / 2.0)), Math.Min((int) byte.MaxValue, (int) Math.Round( this.game.Data.LandscapeTypeObj[index9].Green * 0.5 +  color.G / 2.0)), Math.Min((int) byte.MaxValue, (int) Math.Round( this.game.Data.LandscapeTypeObj[index9].Blue * 0.5 +  color.B / 2.0)));
              }
            }
            if (Index > -1 && useTempAi2 & (this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(Index) < 1 | this.game.Data.Turn == Index & this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon < 1 & this.game.Data.ShrowdOn) && this.game.EditObj.TempAI2[cx, cy] > -1 && this.game.Data.LandscapeTypeObj[this.game.EditObj.TempAI2[cx, cy]].IsSea)
            {
              let mut index10: i32 =  this.game.EditObj.TempAI2[cx, cy];
              color = Color.FromArgb((int) byte.MaxValue, this.game.Data.LandscapeTypeObj[index10].Red, this.game.Data.LandscapeTypeObj[index10].Green, this.game.Data.LandscapeTypeObj[index10].Blue);
            }
            if (highlightTempvar4 > 0 & (!flag4 | flag6) && this.game.EditObj.TempValue4[0].Value[cx, cy] == highlightTempvar4)
            {
              let mut red: i32 =  Math.Min((int) byte.MaxValue, (int) color.R + 50);
              let mut green: i32 =  Math.Min((int) byte.MaxValue, (int) color.G + 50);
              let mut blue: i32 =  Math.Min((int) byte.MaxValue, (int) color.B + 50);
              color = Color.FromArgb((int) color.A, red, green, blue);
            }
            if (useTempVar3asAlpha && this.game.EditObj.TempValue3[0].Value[cx, cy] < (int) byte.MaxValue)
            {
              let mut red: i32 =  (int) Math.Round( color.R * ((Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) - 1.0) / 32.0));
              let mut green: i32 =  (int) Math.Round( color.G * (Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) / 32.0));
              let mut blue: i32 =  (int) Math.Round( color.B * (Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) / 32.0));
              if (red < 0)
                red = 0;
              if (green < 0)
                green = 0;
              if (blue < 0)
                blue = 0;
              color = Color.FromArgb((int) color.A, red, green, blue);
            }
            if (index1 > -1 && this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1) > -1 & !useTempAi2)
            {
              let mut red: i32 =  (int) Math.Round(Math.Floor( ((int) color.R * 2 + this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1)].Red) / 3.0));
              let mut green: i32 =  (int) Math.Round(Math.Floor( ((int) color.G * 2 + this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1)].Green) / 3.0));
              let mut blue: i32 =  (int) Math.Round(Math.Floor( ((int) color.B * 2 + this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1)].Blue) / 3.0));
              color = Color.FromArgb((int) color.A, red, green, blue);
            }
            if (!this.game.Data.ShrowdOn)
              flag5 = true;
            if (!realhex)
            {
              DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
            }
            else
            {
              let mut num20: i32 =  predrawn ? 1 : 0;
              if ((alsoshading & predrawn | !predrawn & alsoshading) & specialMode1 == -1)
              {
                if (this.game.EditObj.se1_StrategySpecial2 == 1 & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime != index1 & flag5)
                {
                  let mut val1_1: i32 =  0;
                  let mut val1_2: i32 =  0;
                  let mut val1_3: i32 =  0;
                  index8 = -1;
                  if (this.game.EditObj.TempAI[cx, cy] > -1)
                    index8 = this.game.EditObj.TempAI[cx, cy];
                  if (index8 > -1)
                  {
                    let mut alpha: i32 =  75;
                    if (flagArray[index8])
                    {
                      if (numArray1[index8] >= 30)
                      {
                        val1_1 = 128 - Math.Min(128, (int) Math.Round(128.0 * ( (numArray1[index8] - 30) / 30.0)));
                        val1_2 = 128 + Math.Min(128, (int) Math.Round(128.0 * ( (numArray1[index8] - 30) / 30.0)));
                      }
                      else
                      {
                        val1_1 = (int) byte.MaxValue - Math.Min(128, (int) Math.Round(128.0 * ( numArray1[index8] / 30.0)));
                        val1_2 = 128;
                      }
                    }
                    else
                    {
                      if (numArray2[index8] == 1)
                        alpha = 150;
                      if (this.game.Data.RegimeObj[index1].RegimeRel[index8] >= 1)
                        val1_2 = (int) byte.MaxValue - Math.Min(128, (int) Math.Round(128.0 * ( numArray1[index8] / 60.0)));
                      else if (this.game.Data.RegimeObj[index1].RegimeRel[index8] == 0)
                        val1_1 = (int) byte.MaxValue - Math.Min(128, (int) Math.Round(128.0 * ( numArray1[index8] / 60.0)));
                    }
                    if (index8 < 2)
                    {
                      val1_1 = 0;
                      val1_2 = 0;
                      val1_3 = 0;
                    }
                    let mut red1: i32 =  Math.Max(0, Math.Min(val1_1, (int) byte.MaxValue));
                    let mut green1: i32 =  Math.Max(0, Math.Min(val1_2, (int) byte.MaxValue));
                    let mut blue1: i32 =  Math.Max(0, Math.Min(val1_3, (int) byte.MaxValue));
                    color = Color.FromArgb(alpha, red1, green1, blue1);
                    if (useTempVar3asAlpha && this.game.EditObj.TempValue3[0].Value[cx, cy] < (int) byte.MaxValue)
                    {
                      let mut red2: i32 =  (int) Math.Round( color.R * ((Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) - 1.0) / 32.0));
                      let mut green2: i32 =  (int) Math.Round( color.G * (Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) / 32.0));
                      let mut blue2: i32 =  (int) Math.Round( color.B * (Math.Sqrt( this.game.EditObj.TempValue3[0].Value[cx, cy]) / 32.0));
                      if (red2 < 0)
                        red2 = 0;
                      if (green2 < 0)
                        green2 = 0;
                      if (blue2 < 0)
                        blue2 = 0;
                      color = Color.FromArgb((int) color.A, red2, green2, blue2);
                    }
                    if (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon < 1 & this.game.Data.ShrowdOn & this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1) == -1 & this.game.Data.ShrowdOn)
                      DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                    DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                  }
                  else
                    DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                }
                else if (flag4)
                {
                  DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                }
                else
                {
                  index8 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime;
                  if (index1 != index8)
                  {
                    if (this.game.Data.MapObj[0].HexObj[cx, cy].get_ReconPts(Index) < 1 & this.game.Data.MapObj[0].HexObj[cx, cy].get_LastReg(Index) == -1 & this.game.Data.ShrowdOn)
                      DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) Math.Round( color.A * 0.5));
                    else if (index8 > -1)
                    {
                      if (flagArray[index8])
                        index8 = 1;
                      color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( color.R * 0.5 + 0.5 *  this.game.Data.RegimeObj[index8].Red), (int) Math.Round( color.G * 0.5 + 0.5 *  this.game.Data.RegimeObj[index8].Green), (int) Math.Round( color.B * 0.5 + 0.5 *  this.game.Data.RegimeObj[index8].Blue));
                      if (this.game.Data.RegimeObj[index8].Red == 0 & this.game.Data.RegimeObj[index8].Blue == 0 & this.game.Data.RegimeObj[index8].Green == 0)
                        DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, 0, 0, 0, 0);
                      else
                        DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) Math.Round( color.A * 0.5));
                    }
                    else
                      DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) color.R, (int) color.G, (int) color.B, (int) Math.Round( color.A * 0.5));
                  }
                  else if (highlightTempvar4 > 0 && this.game.EditObj.TempValue4[0].Value[cx, cy] == highlightTempvar4)
                    DrawMod.DrawBlock(ref Expression, num15, num16, num17, num18, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 60);
                }
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn) && !flag4 & index8 > -1 & showflag & this.game.Data.MapObj[0].HexObj[cx, cy].VP > 0 & this.game.Data.MapObj[0].HexObj[cx, cy].Regime > -1)
                {
                  ref Graphics local15 = ref Expression;
                  bitmap = BitmapStore.GetBitmap(this.game.Data.RegimeObj[this.game.Data.MapObj[0].HexObj[cx, cy].Regime].HQSpriteNr, -1);
                  ref local16: Bitmap = ref bitmap;
                  rectangle2 = Rectangle::new(4, 0, 12, 12);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(num15, num16, num17, num18);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
                }
              }
            }
            let mut num21: i32 =  0;
            let mut num22: i32 =  0;
            if (this.game.Data.Round > 0)
            {
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_LastLT(index1) > -1)
                num21 = 1;
              if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].get_SeeNow(Index) > 0)
                num22 = 1;
              if (this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                num22 = 1;
              if (useTempVar3asAlpha && this.game.EditObj.TempValue3[0].Value[cx, cy] > 0)
                num22 = 1;
            }
            if (!flag3 | num21 == 1 && num17 >= 3 & num18 >= 3 & !predrawn)
            {
              c: Color = Color.FromArgb(105, 90, 90, 150);
              if (num1 > 0)
                c = Color.FromArgb(105, 150, 90, 70);
              let mut index11: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].RiverType[index11] > -1 && !this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].RiverType[index11]].snakeMode)
                {
                  if (index11 == 0)
                    DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c, 2);
                  if (index11 == 1)
                    DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c, 2);
                  if (index11 == 2)
                    DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c, 2);
                  if (index11 == 3)
                    DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c, 2);
                  if (index11 == 4)
                    DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c, 2);
                  if (index11 == 5)
                    DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c, 2);
                }
                index11 += 1;
              }
              while (index11 <= 5);
            }
            if (!flag3 | num22 == 1 && num17 >= 2 & num18 >= 2 & !predrawn)
            {
              c1: Color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              c2: Color = Color.FromArgb((int) byte.MaxValue, 200, 0, 0);
              if (useTempVar3asAlpha)
              {
                if (this.game.EditObj.TempValue3[0].Value[cx, cy] < 999)
                {
                  let mut red3: i32 =  (int) Math.Round( c1.R * (( this.game.EditObj.TempValue3[0].Value[cx, cy] +  ((int) byte.MaxValue - this.game.EditObj.TempValue3[0].Value[cx, cy]) / 2.0) /  byte.MaxValue));
                  if (red3 > 528)
                    red3 = red3;
                  let mut green3: i32 =  (int) Math.Round( c1.G * (( this.game.EditObj.TempValue3[0].Value[cx, cy] +  ((int) byte.MaxValue - this.game.EditObj.TempValue3[0].Value[cx, cy]) / 2.0) /  byte.MaxValue));
                  let mut blue3: i32 =  (int) Math.Round( c1.B * (( this.game.EditObj.TempValue3[0].Value[cx, cy] +  ((int) byte.MaxValue - this.game.EditObj.TempValue3[0].Value[cx, cy]) / 2.0) /  byte.MaxValue));
                  c1 = Color.FromArgb((int) c1.A, red3, green3, blue3);
                  let mut red4: i32 =  (int) Math.Round( c2.R * ( this.game.EditObj.TempValue3[0].Value[cx, cy] /  byte.MaxValue));
                  let mut green4: i32 =  (int) Math.Round( c2.G * ( this.game.EditObj.TempValue3[0].Value[cx, cy] /  byte.MaxValue));
                  let mut blue4: i32 =  (int) Math.Round( c2.B * ( this.game.EditObj.TempValue3[0].Value[cx, cy] /  byte.MaxValue));
                  c2 = Color.FromArgb((int) c2.A, red4, green4, blue4);
                }
                else
                  c2 = Color.FromArgb((int) byte.MaxValue, 0, 0, 0);
              }
              let mut tfacing: i32 =  1;
              do
              {
                coordinate = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap)
                {
                  let mut num23: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].Regime;
                  let mut regime: i32 =  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime;
                  if (useTempVar3asAlpha)
                  {
                    num23 = this.game.EditObj.TempAI[cx, cy];
                    regime = this.game.EditObj.TempAI[coordinate.x, coordinate.y];
                    if (this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                    {
                      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
                        num23 = -1;
                    }
                    else if (this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1) > -1)
                    {
                      if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1)].IsSea)
                        num23 = -1;
                    }
                    else if (useTempAi2 && this.game.EditObj.TempAI2[cx, cy] > -1 && this.game.Data.LandscapeTypeObj[this.game.EditObj.TempAI2[cx, cy]].IsSea)
                      num23 = -1;
                  }
                  if (cx == 48 & cy == 34)
                    cx = cx;
                  if (num23 > -1 & regime > -1)
                  {
                    if (num23 != regime)
                    {
                      if (highlightTempvar4 > 0)
                      {
                        if (highlightTempvar4 > 0 & (this.game.EditObj.TempValue4[0].Value[cx, cy] == highlightTempvar4 | this.game.EditObj.TempValue4[0].Value[coordinate.x, coordinate.y] == highlightTempvar4))
                        {
                          if (tfacing == 1)
                            DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c1, 3);
                          if (tfacing == 2)
                            DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c1, 3);
                          if (tfacing == 3)
                            DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c1, 3);
                          if (tfacing == 4)
                            DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c1, 3);
                          if (tfacing == 5)
                            DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c1, 3);
                          if (tfacing == 6)
                            DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c1, 3);
                        }
                        else
                        {
                          if (tfacing == 1)
                            DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c2, 2);
                          if (tfacing == 2)
                            DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c2, 2);
                          if (tfacing == 3)
                            DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c2, 2);
                          if (tfacing == 4)
                            DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c2, 2);
                          if (tfacing == 5)
                            DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c2, 2);
                          if (tfacing == 6)
                            DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c2, 2);
                        }
                      }
                      else
                      {
                        let mut widdy: i32 =  2;
                        if (specialMode1 > -1)
                          widdy = 1;
                        if (tfacing == 1)
                          DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c2, widdy);
                        if (tfacing == 2)
                          DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c2, widdy);
                        if (tfacing == 3)
                          DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c2, widdy);
                        if (tfacing == 4)
                          DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c2, widdy);
                        if (tfacing == 5)
                          DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c2, widdy);
                        if (tfacing == 6)
                          DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c2, widdy);
                      }
                    }
                    else if (specialMode1 == -1)
                    {
                      let mut num24: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].GetHexLibVarValue(libVar9);
                      let mut num25: i32 =  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].GetHexLibVarValue(libVar9);
                      if (useTempZones)
                      {
                        if (useTempVar3asAlpha)
                        {
                          num24 = this.game.EditObj.TempValue4[0].Value[cx, cy];
                          num25 = this.game.EditObj.TempValue4[0].Value[coordinate.x, coordinate.y];
                        }
                      }
                      else if (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon < 1 | this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].MaxRecon < 1 | !this.game.Data.ShrowdOn)
                      {
                        num24 = -1;
                        num25 = -1;
                      }
                      if (num24 != num25 & num24 > 0 & num25 > 0)
                      {
                        let mut num26: i32 =  0;
                        let mut num27: i32 =  0;
                        if (index1 > -1)
                        {
                          if (this.game.Data.MapObj[0].HexObj[cx, cy].GetHexLibVarValue(libVar9) > 0)
                            num26 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData3(0, this.game.Data.RegimeObj[index1].id, 1, this.game.Data.MapObj[0].HexObj[cx, cy].GetHexLibVarValue(libVar9), 2, "recon", 3)));
                          if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].GetHexLibVarValue(libVar9) > 0)
                            num27 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData3(0, this.game.Data.RegimeObj[index1].id, 1, this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].GetHexLibVarValue(libVar9), 2, "recon", 3)));
                          if (useTempZones)
                          {
                            num26 = 0;
                            num27 = 0;
                            if (this.game.EditObj.TempValue4[0].Value[cx, cy] > 0)
                              num26 = this.game.EditObj.TempValue3[0].Value[cx, cy];
                            if (this.game.EditObj.TempValue4[0].Value[coordinate.x, coordinate.y] > 0)
                              num27 = this.game.EditObj.TempValue3[0].Value[coordinate.x, coordinate.y];
                          }
                          bool flag7 = false;
                          if (this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                          {
                            if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].LandscapeType].IsSea)
                              flag7 = true;
                          }
                          else if (this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1) > -1)
                          {
                            if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index1)].IsSea)
                              flag7 = true;
                          }
                          else if (useTempAi2 && this.game.EditObj.TempAI2[cx, cy] > -1 && this.game.Data.LandscapeTypeObj[this.game.EditObj.TempAI2[cx, cy]].IsSea)
                            flag7 = true;
                          if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                          {
                            if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                              flag7 = true;
                          }
                          else if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].get_LastLT(index1) > -1)
                          {
                            if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].get_LastLT(index1)].IsSea)
                              flag7 = true;
                          }
                          else if (useTempAi2 && this.game.EditObj.TempAI2[coordinate.x, coordinate.y] > -1 && this.game.Data.LandscapeTypeObj[this.game.EditObj.TempAI2[coordinate.x, coordinate.y]].IsSea)
                            flag7 = true;
                          if (!flag7 & (!this.game.Data.FOWOn | num26 > 0 | num27 > 0 | this.game.Data.MapObj[0].HexObj[cx, cy].Regime == index1 | this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime == index1))
                          {
                            bool flag8 = false;
                            if (highlightTempvar4 > 0 && (this.game.EditObj.TempValue4[0].Value[cx, cy] == highlightTempvar4 | this.game.EditObj.TempValue4[0].Value[coordinate.x, coordinate.y] == highlightTempvar4) & useTempZones)
                            {
                              if (tfacing == 1)
                                DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c1, 3);
                              if (tfacing == 2)
                                DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c1, 3);
                              if (tfacing == 3)
                                DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c1, 3);
                              if (tfacing == 4)
                                DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c1, 3);
                              if (tfacing == 5)
                                DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c1, 3);
                              if (tfacing == 6)
                                DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c1, 3);
                              flag8 = true;
                            }
                            if (!flag8)
                            {
                              if (tfacing == 1)
                                DrawMod.drawLine(ref Expression, num15, num16, num15 + num17, num16, c2);
                              if (tfacing == 2)
                                DrawMod.drawLine(ref Expression, num15 + num17, num16, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), c2);
                              if (tfacing == 3)
                                DrawMod.drawLine(ref Expression, num15 + num17, (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, num16 + num18, c2);
                              if (tfacing == 4)
                                DrawMod.drawLine(ref Expression, num15, num16 + num18, num15 + num17, num16 + num18, c2);
                              if (tfacing == 5)
                                DrawMod.drawLine(ref Expression, num15, (int) Math.Round( num16 +  num18 / 2.0), num15, num16 + num18, c2);
                              if (tfacing == 6)
                                DrawMod.drawLine(ref Expression, num15, num16, num15, (int) Math.Round( num16 +  num18 / 2.0), c2);
                            }
                          }
                        }
                      }
                    }
                  }
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
            if (!flag3 && num17 >= 8 & num18 >= 8 & !predrawn)
            {
              let mut index12: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].RoadType[index12] > -1)
                {
                  let mut widdy: i32 =  this.game.Data.RoadTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].RoadType[index12]].Thickness;
                  if (widdy < 1)
                    widdy = 1;
                  if (index12 == 0)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), (int) Math.Round( num15 +  num17 / 2.0), num16, 0, 0, 0, 105, widdy);
                  if (index12 == 1)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, (int) Math.Round( num16 +  num18 * 0.25), 0, 0, 0, 105, widdy);
                  if (index12 == 2)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), num15 + num17, (int) Math.Round( num16 +  num18 * 0.75), 0, 0, 0, 105, widdy);
                  if (index12 == 3)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), (int) Math.Round( num15 +  num17 / 2.0), num16 + num18, 0, 0, 0, 105, widdy);
                  if (index12 == 4)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), num15, (int) Math.Round( num16 +  num18 * 0.75), 0, 0, 0, 105, widdy);
                  if (index12 == 5)
                    DrawMod.drawLine(ref Expression, (int) Math.Round( num15 +  num17 / 2.0), (int) Math.Round( num16 +  num18 / 2.0), num15, (int) Math.Round( num16 +  num18 * 0.25), 0, 0, 0, 105, widdy);
                }
                index12 += 1;
              }
              while (index12 <= 5);
            }
          }
        }
      }
      let mut mapWidth2: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
      for (let mut index13: i32 =  0; index13 <= mapWidth2; index13 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
        for (let mut index14: i32 =  0; index14 <= mapHeight; index14 += 1)
        {
          let mut index15: i32 =  index13;
          let mut index16: i32 =  index14;
          if (this.game.EditObj.MiniMapOffset > 0)
          {
            index15 += this.game.EditObj.MiniMapOffset;
            if (index15 > this.game.Data.MapObj[0].MapWidth)
              index15 -= this.game.Data.MapObj[0].MapWidth + 1;
          }
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].LandscapeType > -1)
          {
            num28: i32;
            num29: i32;
            num30: i32;
            num31: i32;
            if (!realhex)
            {
              num28 = (int) Math.Round( (Conversion.Int(num12 *  index13) +  num6));
              num29 = (int) Math.Round( (Conversion.Int(num13 *  index14) - d2 / 2f +  num7));
              if ((index13 + 10) % 2 > 0)
                num29 = (int) Math.Round( ( num29 + d2 / 2f));
              num30 = (int) Math.Round( num12);
              num31 = (int) Math.Round( num13);
            }
            else
            {
              num28 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index13 +  num6)));
              num29 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index14) - Math.Floor( d2 / 2.0) +  num7));
              if ((index13 + 10) % 2 > 0)
                num29 = (int) Math.Round( num29 + Math.Floor( d2 / 2.0));
              num30 = (int) Math.Round(Math.Floor( num12));
              num31 = (int) Math.Round(Math.Floor( num13));
            }
            flag1 = false;
            if (!this.game.Data.ShrowdOn && !flag3 && alsounits & specialMode1 == -1)
            {
              let mut num32: i32 =  0;
              let mut unitCounter: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitCounter;
              index17: i32;
              for (index17 = 0; index17 <= unitCounter; index17 += 1)
              {
                if (this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitList[index17], index1) > 0)
                {
                  num32 = 1;
                  break;
                }
              }
              if (num32 == 1)
              {
                let mut index18: i32 =  this.game.Data.UnitObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitList[index17]].Regime;
                if (Operators.ConditionalCompareObjectGreater(this.game.HandyFunctionsObj.GetUnitPeople(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitList[index17]),  -1, false) && this.game.Data.PeopleObj[Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitList[index17]))].RegCol > -1)
                  index18 = this.game.Data.PeopleObj[Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index15, index16].UnitList[index17]))].RegCol;
                let mut r: i32 =  this.game.Data.RegimeObj[index18].Red - 50;
                let mut g: i32 =  this.game.Data.RegimeObj[index18].Green - 50;
                let mut b: i32 =  this.game.Data.RegimeObj[index18].Blue - 50;
                if (r < 0)
                  r = 0;
                if (g < 0)
                  g = 0;
                if (b < 0)
                  b = 0;
                DrawMod.DrawBlock(ref Expression, (int) Math.Round( num28 +  num30 * 0.25), (int) Math.Round( num29 +  num31 * 0.25), (int) Math.Round( num30 * 0.5), (int) Math.Round( num31 * 0.5), r, g, b, (int) byte.MaxValue);
                DrawMod.DrawRectangle(ref Expression, (int) Math.Round( num28 +  num30 * 0.25), (int) Math.Round( num29 +  num31 * 0.25), (int) Math.Round( num30 * 0.5), (int) Math.Round( num31 * 0.5), this.game.Data.RegimeObj[index18].Red * 2, this.game.Data.RegimeObj[index18].Green * 2, this.game.Data.RegimeObj[index18].Blue * 2, (int) byte.MaxValue);
              }
            }
          }
        }
      }
      if (specialMode1 > 0)
      {
        let mut mapWidth3: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut index19: i32 =  0; index19 <= mapWidth3; index19 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut index20: i32 =  0; index20 <= mapHeight; index20 += 1)
          {
            let mut index21: i32 =  index19;
            let mut index22: i32 =  index20;
            if (this.game.EditObj.MiniMapOffset > 0)
            {
              index21 += this.game.EditObj.MiniMapOffset;
              if (index21 > this.game.Data.MapObj[0].MapWidth)
                index21 -= this.game.Data.MapObj[0].MapWidth + 1;
            }
            let mut num33: i32 =  1;
            if (this.game.Data.ShrowdOn & !this.game.Data.ShrowdPeek)
            {
              num33 = 0;
              if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index21, index22].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[index21, index22].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                num33 = 1;
            }
            if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index21, index22].get_SeeNow(Index) < 1 | this.game.Data.Turn == Index & this.game.Data.MapObj[0].HexObj[index21, index22].MaxRecon < 1 & this.game.Data.ShrowdOn && this.game.Data.ShrowdPeek)
              num33 = 0;
            num34: i32;
            num35: i32;
            if (num33 == 1)
            {
              bool flag9 = false;
              bool flag10 = false;
              index23: i32;
              if (specialMode1 >= 1 & specialMode1 <= 6)
              {
                let mut hexLibVarValue1: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar2);
                let mut hexLibVarValue2: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar1);
                let mut hexLibVarValue3: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar3);
                if (hexLibVarValue1 <= 0 & hexLibVarValue3 > 0 && hexLibVarValue2 == specialMode1 | hexLibVarValue2 > 0 & specialMode1 == 6)
                {
                  flag9 = true;
                  index23 = -1;
                  bool flag11 = false;
                  if (this.game.Data.MapObj[0].HexObj[index21, index22].Regime == this.game.Data.Turn | this.game.Data.MapObj[0].HexObj[index21, index22].MaxRecon > 0 | !this.game.Data.ShrowdOn)
                  {
                    let mut length: i32 =  this.game.Data.StringListObj[stringListById5].Length;
                    for (let mut index24: i32 =  0; index24 <= length; index24 += 1)
                    {
                      if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[index24, 3])) == index21 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[index24, 4])) == index22)
                      {
                        let mut idValue: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[index24, 1]));
                        if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue, 4))) == 2 & (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue, 9))) == hexLibVarValue2)
                          flag11 = true;
                      }
                    }
                  }
                  if (!flag11)
                  {
                    if (hexLibVarValue2 == 2)
                      index23 = this.game.Data.FindSmallPic(112, "SE_Graphics");
                    if (hexLibVarValue2 == 3)
                      index23 = this.game.Data.FindSmallPic(147, "SE_Graphics");
                    if (hexLibVarValue2 == 5)
                      index23 = this.game.Data.FindSmallPic(113, "SE_Graphics");
                    if (hexLibVarValue2 == 4)
                      index23 = this.game.Data.FindSmallPic(163, "SE_Graphics");
                    if (hexLibVarValue2 == 1)
                      index23 = this.game.Data.FindSmallPic(152, "SE_Graphics");
                  }
                  else
                  {
                    if (hexLibVarValue2 == 2)
                      index23 = this.game.Data.FindSmallPic(114, "SE_Graphics");
                    if (hexLibVarValue2 == 3)
                      index23 = this.game.Data.FindSmallPic(148, "SE_Graphics");
                    if (hexLibVarValue2 == 5)
                      index23 = this.game.Data.FindSmallPic(115, "SE_Graphics");
                    if (hexLibVarValue2 == 4)
                      index23 = this.game.Data.FindSmallPic(164, "SE_Graphics");
                    if (hexLibVarValue2 == 1)
                      index23 = this.game.Data.FindSmallPic(154, "SE_Graphics");
                  }
                }
              }
              num36: i32;
              num37: i32;
              num38: i32;
              num39: i32;
              color: Color;
              if (specialMode1 == 7)
              {
                let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar4);
                if (hexLibVarValue > 0)
                {
                  if (hexLibVarValue < 500)
                  {
                    num36 = (int) Math.Round( byte.MaxValue * ( (500 - hexLibVarValue) / 500.0));
                    num37 = (int) byte.MaxValue;
                    num38 = (int) byte.MaxValue;
                    num39 = (int) Math.Round(128.0 - 128.0 * ( (500 - hexLibVarValue) / 500.0));
                  }
                  else if (hexLibVarValue >= 500 & hexLibVarValue < 1000)
                  {
                    num36 = 0;
                    num37 = (int) byte.MaxValue - (int) Math.Round( byte.MaxValue * ( (hexLibVarValue - 500) / 500.0));
                    num38 = (int) byte.MaxValue;
                    num39 = 128;
                  }
                  else if (hexLibVarValue >= 1000)
                  {
                    num36 = 0;
                    num37 = 0;
                    num38 = (int) byte.MaxValue - (int) Math.Round( byte.MaxValue * ( (hexLibVarValue - 1000) / 6000.0));
                    num39 = 128;
                  }
                  num36 = Math.Min((int) byte.MaxValue, Math.Max(num36, 0));
                  num37 = Math.Min((int) byte.MaxValue, Math.Max(num37, 0));
                  num38 = Math.Min((int) byte.MaxValue, Math.Max(num38, 0));
                  num39 = Math.Min((int) byte.MaxValue, Math.Max(num39, 0));
                  flag10 = true;
                  color = Color.FromArgb(num39, num36, num37, num38);
                  if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[index21, index22].LandscapeType].IsSea)
                    flag10 = false;
                }
              }
              if (specialMode1 == 8)
              {
                let mut num40: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar5) + 273;
                if (num40 > 0)
                {
                  if (num40 < 173)
                  {
                    num36 = 0;
                    num37 = 0;
                    num38 = (int) byte.MaxValue;
                    num39 = 128;
                  }
                  else if (num40 < 273)
                  {
                    num36 = 0;
                    num37 = (int) Math.Round( byte.MaxValue * ( (num40 - 173) / 100.0));
                    num38 = (int) byte.MaxValue;
                    num39 = 128;
                  }
                  else if (num40 < 283)
                  {
                    num36 = (int) Math.Round( byte.MaxValue * ( (num40 - 273) / 10.0));
                    num37 = (int) byte.MaxValue;
                    num38 = (int) byte.MaxValue - (int) Math.Round( byte.MaxValue * ( (num40 - 273) / 10.0));
                    num39 = 128;
                  }
                  else if (num40 < 323)
                  {
                    num36 = (int) byte.MaxValue;
                    num37 = (int) byte.MaxValue - (int) Math.Round( byte.MaxValue * ( (num40 - 283) / 40.0));
                    num38 = 0;
                    num39 = 128;
                  }
                  else if (num40 >= 323)
                  {
                    num36 = (int) byte.MaxValue - (int) Math.Round( byte.MaxValue * ( (num40 - 323) / 50.0));
                    num37 = 0;
                    num38 = 0;
                    num39 = 128;
                  }
                  num36 = Math.Min((int) byte.MaxValue, Math.Max(num36, 0));
                  num37 = Math.Min((int) byte.MaxValue, Math.Max(num37, 0));
                  num38 = Math.Min((int) byte.MaxValue, Math.Max(num38, 0));
                  num39 = Math.Min((int) byte.MaxValue, Math.Max(num39, 0));
                  flag10 = true;
                  color = Color.FromArgb(num39, num36, num37, num38);
                  if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[index21, index22].LandscapeType].IsSea)
                    flag10 = false;
                }
              }
              if (specialMode1 == 9)
              {
                let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar6);
                if (hexLibVarValue > 0)
                {
                  if (hexLibVarValue < 500)
                  {
                    num36 = (int) byte.MaxValue;
                    num37 = 128;
                    num38 = 0;
                    num39 = (int) Math.Round(128.0 - 128.0 * ( (500 - hexLibVarValue) / 500.0));
                  }
                  else if (hexLibVarValue >= 500 & hexLibVarValue < 2000)
                  {
                    num36 = (int) byte.MaxValue - (int) Math.Round(128.0 * ( (hexLibVarValue - 500) / 1500.0));
                    num37 = 128 - (int) Math.Round(64.0 * ( (hexLibVarValue - 500) / 1500.0));
                    num38 = 0;
                    num39 = 128;
                  }
                  else if (hexLibVarValue >= 2000)
                  {
                    num36 = 128;
                    num37 = 64;
                    num38 = 0;
                    num39 = 128 + (int) Math.Round( (64 * (hexLibVarValue - 2000)) / 2000.0);
                  }
                  num36 = Math.Min((int) byte.MaxValue, Math.Max(num36, 0));
                  num37 = Math.Min((int) byte.MaxValue, Math.Max(num37, 0));
                  num38 = Math.Min((int) byte.MaxValue, Math.Max(num38, 0));
                  num39 = Math.Min((int) byte.MaxValue, Math.Max(num39, 0));
                  flag10 = true;
                  color = Color.FromArgb(num39, num36, num37, num38);
                  if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[index21, index22].LandscapeType].IsSea)
                    flag10 = false;
                }
              }
              if (specialMode1 == 10)
              {
                let mut heightLevel: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].HeightLevel;
                if (heightLevel < 1)
                {
                  num36 = 20;
                  num37 = 10;
                  num38 = 0;
                  num39 = 150;
                }
                else if (heightLevel >= 1 & heightLevel < 9)
                {
                  num36 = (int) Math.Round( byte.MaxValue * ( heightLevel / 8.0));
                  num37 = (int) Math.Round(150.0 * ( heightLevel / 8.0));
                  num38 = (int) Math.Round(50.0 * ( heightLevel / 8.0));
                  num39 = 150 - (int) Math.Round(100.0 * ( heightLevel / 8.0));
                }
                else if (heightLevel >= 9)
                {
                  num36 = (int) byte.MaxValue;
                  num37 = 200;
                  num38 = 150;
                  num39 = 50;
                }
                num36 = Math.Min((int) byte.MaxValue, Math.Max(num36, 0));
                num37 = Math.Min((int) byte.MaxValue, Math.Max(num37, 0));
                num38 = Math.Min((int) byte.MaxValue, Math.Max(num38, 0));
                num39 = (int) Math.Round( Math.Min((int) byte.MaxValue, Math.Max(num39, 0)) * 0.75);
                flag10 = true;
                color = Color.FromArgb(num39, num36, num37, num38);
                if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[index21, index22].LandscapeType].IsSea)
                  flag10 = false;
              }
              if (specialMode1 == 11)
              {
                let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar7);
                if (hexLibVarValue > 0)
                {
                  Random random = new Random(hexLibVarValue);
                  num36 = random.Next(50, (int) byte.MaxValue);
                  num37 = random.Next(50, (int) byte.MaxValue);
                  num38 = random.Next(50, (int) byte.MaxValue);
                  num39 = 150;
                  flag10 = true;
                  color = Color.FromArgb(num39, num36, num37, num38);
                }
              }
              if (specialMode1 == 12)
              {
                let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index21, index22].GetHexLibVarValue(libVar8);
                if (hexLibVarValue > 0)
                {
                  let mut val1_4: i32 =  200 - (int) Math.Round(100.0 * ( hexLibVarValue / 20000.0));
                  let mut val1_5: i32 =  0;
                  let mut val1_6: i32 =  200 - (int) Math.Round(100.0 * ( hexLibVarValue / 20000.0));
                  let mut val1_7: i32 =  64 + Math.Min(64, (int) Math.Round(64.0 * ( hexLibVarValue / 20000.0)));
                  num36 = Math.Min((int) byte.MaxValue, Math.Max(val1_4, 0));
                  num37 = Math.Min((int) byte.MaxValue, Math.Max(val1_5, 0));
                  num38 = Math.Min((int) byte.MaxValue, Math.Max(val1_6, 0));
                  num39 = Math.Min((int) byte.MaxValue, Math.Max(val1_7, 0));
                  flag10 = true;
                  color = Color.FromArgb(num39, num36, num37, num38);
                  if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[index21, index22].LandscapeType].IsSea)
                    flag10 = false;
                }
              }
              if (flag9 | flag10)
              {
                let mut num41: i32 =  index19;
                num34 = index20;
                if (this.game.EditObj.MiniMapOffset > 0)
                {
                  let mut num42: i32 =  num41 + this.game.EditObj.MiniMapOffset;
                  if (num42 > this.game.Data.MapObj[0].MapWidth)
                    num35 = num42 - (this.game.Data.MapObj[0].MapWidth + 1);
                }
                x1: i32;
                y1: i32;
                w: i32;
                h: i32;
                if (!realhex)
                {
                  x1 = (int) Math.Round( (Conversion.Int(num12 *  index19) +  num6));
                  y1 = (int) Math.Round( (Conversion.Int(num13 *  index20) - d2 / 2f +  num7));
                  if ((index19 + 10) % 2 > 0)
                    y1 = (int) Math.Round( ( y1 + d2 / 2f));
                  w = (int) Math.Round( (num12 + 1f));
                  h = (int) Math.Round( (num13 + 1f));
                }
                else
                {
                  x1 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index19 +  num6)));
                  y1 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index20) - Math.Floor( d2 / 2.0) +  num7));
                  if ((index19 + 10) % 2 > 0)
                    y1 = (int) Math.Round( y1 + Math.Floor( d2 / 2.0));
                  w = (int) Math.Round(Math.Floor( num12));
                  h = (int) Math.Round(Math.Floor( num13));
                }
                if (flag10)
                  DrawMod.DrawBlock(ref Expression, x1, y1, w, h, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                else if (flag9 && index23 > -1)
                {
                  ref Graphics local17 = ref Expression;
                  bitmap = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[index23]);
                  ref local18: Bitmap = ref bitmap;
                  let mut x: i32 =  x1 - 36;
                  let mut y: i32 =  y1 - 20;
                  DrawMod.DrawSimple(ref local17, ref local18, x, y);
                }
              }
            }
            else if (this.game.Data.MapObj[0].HexObj[index21, index22].get_LastLT(Index) > -1)
            {
              let mut num43: i32 =  index19;
              num34 = index20;
              if (this.game.EditObj.MiniMapOffset > 0)
              {
                let mut num44: i32 =  num43 + this.game.EditObj.MiniMapOffset;
                if (num44 > this.game.Data.MapObj[0].MapWidth)
                  num35 = num44 - (this.game.Data.MapObj[0].MapWidth + 1);
              }
              x1: i32;
              y1: i32;
              w: i32;
              h: i32;
              if (!realhex)
              {
                x1 = (int) Math.Round( (Conversion.Int(num12 *  index19) +  num6));
                y1 = (int) Math.Round( (Conversion.Int(num13 *  index20) - d2 / 2f +  num7));
                if ((index19 + 10) % 2 > 0)
                  y1 = (int) Math.Round( ( y1 + d2 / 2f));
                w = (int) Math.Round( (num12 + 1f));
                h = (int) Math.Round( (num13 + 1f));
              }
              else
              {
                x1 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index19 +  num6)));
                y1 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index20) - Math.Floor( d2 / 2.0) +  num7));
                if ((index19 + 10) % 2 > 0)
                  y1 = (int) Math.Round( y1 + Math.Floor( d2 / 2.0));
                w = (int) Math.Round(Math.Floor( num12));
                h = (int) Math.Round(Math.Floor( num13));
              }
              DrawMod.DrawBlock(ref Expression, x1, y1, w, h, 0, 0, 0, 100);
            }
          }
        }
      }
      if (((!realhex ? 1 : 0) | 1) != 0)
      {
        let mut mapWidth4: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut index25: i32 =  0; index25 <= mapWidth4; index25 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut index26: i32 =  0; index26 <= mapHeight; index26 += 1)
          {
            let mut index27: i32 =  index25;
            let mut index28: i32 =  index26;
            if (this.game.EditObj.MiniMapOffset > 0)
            {
              index27 += this.game.EditObj.MiniMapOffset;
              if (index27 > this.game.Data.MapObj[0].MapWidth)
                index27 -= this.game.Data.MapObj[0].MapWidth + 1;
            }
            let mut num45: i32 =  1;
            if (this.game.Data.ShrowdOn & !this.game.Data.ShrowdPeek)
            {
              num45 = 0;
              if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index27, index28].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[index27, index28].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                num45 = 1;
            }
            if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index27, index28].get_SeeNow(Index) < 1 | this.game.Data.Turn == Index & this.game.Data.MapObj[0].HexObj[index27, index28].MaxRecon < 1 & this.game.Data.ShrowdOn && this.game.Data.ShrowdPeek)
              num45 = 0;
            num46: i32;
            num47: i32;
            if (num45 == 1)
            {
              if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index27, index28].VP >=  this.game.Data.RuleVar[148] &  this.game.Data.RuleVar[148] != 0.0)
              {
                num48: i32;
                num49: i32;
                if (!realhex)
                {
                  num46 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6));
                  let mut num50: i32 =  (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( ( num50 + d2 / 2f));
                  num48 = (int) Math.Round( num12);
                  num49 = (int) Math.Round( num13);
                }
                else
                {
                  num46 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index25 +  num6)));
                  let mut num51: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( num51 + Math.Floor( d2 / 2.0));
                  num48 = (int) Math.Round(Math.Floor( num12));
                  num49 = (int) Math.Round(Math.Floor( num13));
                }
                let mut w: i32 =  num48 * 1;
                let mut h: i32 =  num49 * 1;
                if (w < 8)
                  w = 8;
                if (h < 8)
                  h = 8;
                x1: i32;
                y1: i32;
                if (realhex)
                {
                  x1 = (int) Math.Round(Conversion.Int(Math.Floor( num12) *  index25) +  num6 +  (int) Math.Round(Math.Floor(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round(Conversion.Int(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7 +  (int) Math.Round(Math.Floor(( num13 -  h) / 2.0)));
                }
                else
                {
                  x1 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6 +  (int) Math.Round(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7 +  (int) Math.Round(( num13 -  h) / 2.0)));
                }
                if ((index25 + 10) % 2 > 0)
                  y1 = (int) Math.Round( ( y1 + d2 / 2f));
                DrawMod.DrawBlock(ref Expression, x1, y1, w, h, 0, 0, 0, (int) byte.MaxValue);
                DrawMod.DrawRectangle(ref Expression, x1, y1, w, h, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              }
              else if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[index27, index28].VP >=  this.game.Data.RuleVar[149] &  this.game.Data.RuleVar[149] != 0.0)
              {
                num52: i32;
                num53: i32;
                if (!realhex)
                {
                  num46 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6));
                  let mut num54: i32 =  (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( ( num54 + d2 / 2f));
                  num52 = (int) Math.Round( num12);
                  num53 = (int) Math.Round( num13);
                }
                else
                {
                  num46 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index25 +  num6)));
                  let mut num55: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( num55 + Math.Floor( d2 / 2.0));
                  num52 = (int) Math.Round(Math.Floor( num12));
                  num53 = (int) Math.Round(Math.Floor( num13));
                }
                let mut w: i32 =  (int) Math.Round( num52 * 0.5);
                let mut h: i32 =  (int) Math.Round( num53 * 0.5);
                if (w < 4)
                  w = 4;
                if (h < 4)
                  h = 4;
                x1: i32;
                y1: i32;
                if (realhex)
                {
                  x1 = (int) Math.Round(Conversion.Int(Math.Floor( num12) *  index25) +  num6 +  (int) Math.Round(Math.Floor(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round(Conversion.Int(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7 +  (int) Math.Round(Math.Floor(( num13 -  h) / 2.0)));
                }
                else
                {
                  x1 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6 +  (int) Math.Round(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7 +  (int) Math.Round(( num13 -  h) / 2.0)));
                }
                if ((index25 + 10) % 2 > 0)
                  y1 = (int) Math.Round( ( y1 + d2 / 2f));
                DrawMod.DrawBlock(ref Expression, x1, y1, w, h, 0, 0, 0, (int) byte.MaxValue);
                DrawMod.DrawRectangle(ref Expression, x1, y1, w, h, 220, 220, 220, (int) byte.MaxValue);
              }
              else if (this.game.Data.MapObj[0].HexObj[index27, index28].Location > -1 & tempBmp.Width > 300)
              {
                num56: i32;
                num57: i32;
                if (!realhex)
                {
                  num46 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6));
                  let mut num58: i32 =  (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( ( num58 + d2 / 2f));
                  num56 = (int) Math.Round( num12);
                  num57 = (int) Math.Round( num13);
                }
                else
                {
                  num46 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index25 +  num6)));
                  let mut num59: i32 =  (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7));
                  if ((index25 + 10) % 2 > 0)
                    num47 = (int) Math.Round( num59 + Math.Floor( d2 / 2.0));
                  num56 = (int) Math.Round(Math.Floor( num12));
                  num57 = (int) Math.Round(Math.Floor( num13));
                }
                let mut w: i32 =  (int) Math.Round( num56 * 0.33);
                let mut h: i32 =  (int) Math.Round( num57 * 0.33);
                if (w < 3)
                  w = 3;
                if (h < 3)
                  h = 3;
                x1: i32;
                y1: i32;
                if (realhex)
                {
                  x1 = (int) Math.Round(Conversion.Int(Math.Floor( num12) *  index25) +  num6 +  (int) Math.Round(Math.Floor(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round(Conversion.Int(Math.Floor( num13) *  index26) - Math.Floor( d2 / 2.0) +  num7 +  (int) Math.Round(Math.Floor(( num13 -  h) / 2.0)));
                }
                else
                {
                  x1 = (int) Math.Round( (Conversion.Int(num12 *  index25) +  num6 +  (int) Math.Round(( num12 -  w) / 2.0)));
                  y1 = (int) Math.Round( (Conversion.Int(num13 *  index26) - d2 / 2f +  num7 +  (int) Math.Round(( num13 -  h) / 2.0)));
                }
                if ((index25 + 10) % 2 > 0)
                  y1 = (int) Math.Round( ( y1 + d2 / 2f));
                DrawMod.DrawBlock(ref Expression, x1, y1, w, h, 0, 0, 0, (int) byte.MaxValue);
                DrawMod.DrawRectangle(ref Expression, x1, y1, w, h, 220, 220, 220, (int) byte.MaxValue);
              }
            }
          }
        }
      }
      if ( this.game.Data.RuleVar[148] > 0.0 |  this.game.Data.RuleVar[149] > 0.0 && realhex | alsounits && predrawn | alsounits)
      {
        let mut mapWidth5: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        num60: i32;
        for (let mut index29: i32 =  0; index29 <= mapWidth5; index29 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut index30: i32 =  0; index30 <= mapHeight; index30 += 1)
          {
            let mut x: i32 =  index29;
            let mut y: i32 =  index30;
            if (this.game.EditObj.MiniMapOffset > 0)
            {
              x += this.game.EditObj.MiniMapOffset;
              if (x > this.game.Data.MapObj[0].MapWidth)
                x -= this.game.Data.MapObj[0].MapWidth + 1;
            }
            let mut num61: i32 =  1;
            if (this.game.Data.ShrowdOn & !this.game.Data.ShrowdPeek)
            {
              num61 = 0;
              if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[x, y].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                num61 = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].get_SeeNow(Index) < 1 | this.game.Data.Turn == Index & this.game.Data.MapObj[0].HexObj[x, y].MaxRecon < 1 & this.game.Data.ShrowdOn && this.game.Data.ShrowdPeek &  this.game.Data.RuleVar[874] == 1.0)
              num61 = 0;
            if (!realhex & alsounits)
              num61 = 0;
            if (this.game.Data.MapObj[0].HexObj[x, y].MaxRecon < 1 & this.game.Data.ShrowdOn)
              num61 = 0;
            if (num61 == 1 && !( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP >=  this.game.Data.RuleVar[148] &  this.game.Data.RuleVar[148] != 0.0) &&  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP >=  this.game.Data.RuleVar[149] &  this.game.Data.RuleVar[149] != 0.0)
            {
              num62: i32;
              num63: i32;
              num64: i32;
              num65: i32;
              if (!realhex)
              {
                num62 = (int) Math.Round( (Conversion.Int(num12 *  index29) +  num6));
                num63 = (int) Math.Round( (Conversion.Int(num13 *  index30) - d2 / 2f +  num7));
                if ((index29 + 10) % 2 > 0)
                  num63 = (int) Math.Round( ( num63 + d2 / 2f));
                num64 = (int) Math.Round( num12);
                num65 = (int) Math.Round( num13);
              }
              else
              {
                num62 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index29 +  num6)));
                num63 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index30) - Math.Floor( d2 / 2.0) +  num7));
                if ((index29 + 10) % 2 > 0)
                  num63 = (int) Math.Round( num63 + Math.Floor( d2 / 2.0));
                num64 = (int) Math.Round(Math.Floor( num12));
                num65 = (int) Math.Round(Math.Floor( num13));
              }
              let mut num66: i32 =  (int) Math.Round( num62 -  num64 / 2.0);
              let mut num67: i32 =  (int) Math.Round( num63 -  num65 / 2.0);
              num60 = num64 * 2;
              let mut num68: i32 =  num65 * 2;
              if (num67 < 1)
                num67 = 1;
              if ( num66 -  sizeF.Width / 2.0 < 2.0)
                num66 = (int) Math.Round( ( num66 + sizeF.Width / 2f));
              if ( num66 +  sizeF.Width / 2.0 >  bwidth)
                num66 = (int) Math.Round( ( num66 - sizeF.Width / 2f));
              str7: String = Strings.UCase(this.game.HandyFunctionsObj.GetHexName(x, y, this.game.EditObj.MapSelected));
              sizeF =  this.game.Data.RuleVar[839] >= 1.0 ? Expression.MeasureString(str7, this.game.MarcFont10) : Expression.MeasureString(str7, this.game.VicFont4);
              let mut num69: i32 =  (int) Math.Round( ( num66 - sizeF.Width / 2f));
              if ( num69 +  sizeF.Width >  tempBmp.Width)
                num69 = (int) Math.Round( ( num69 - ( num69 + sizeF.Width -  tempBmp.Width)));
              if (num69 < 0)
                num69 += -num69;
              if ( this.game.Data.RuleVar[839] < 1.0)
              {
                DrawMod.DrawBlock(ref Expression, num69, (int) Math.Round( num67 +  num68 / 2.0), (int) Math.Round( sizeF.Width), (int) Math.Round( sizeF.Height), 0, 0, 0, 196);
                DrawMod.DrawTextColouredMarc(ref Expression, str7, this.game.VicFont4, num69, (int) Math.Round( num67 +  num68 / 2.0), Color.White);
              }
              else
              {
                DrawMod.DrawBlock(ref Expression, num69, (int) Math.Round( num67 +  num68 / 2.0), (int) Math.Round( sizeF.Width), (int) Math.Round( sizeF.Height), 0, 0, 0, 128);
                DrawMod.DrawTextColouredMarc(ref Expression, str7, this.game.MarcFont10, num69, (int) Math.Round( num67 +  num68 / 2.0), Color.White);
              }
            }
          }
        }
        let mut mapWidth6: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut index31: i32 =  0; index31 <= mapWidth6; index31 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut index32: i32 =  0; index32 <= mapHeight; index32 += 1)
          {
            let mut x: i32 =  index31;
            let mut y: i32 =  index32;
            if (this.game.EditObj.MiniMapOffset > 0)
            {
              x += this.game.EditObj.MiniMapOffset;
              if (x > this.game.Data.MapObj[0].MapWidth)
                x -= this.game.Data.MapObj[0].MapWidth + 1;
            }
            let mut num70: i32 =  1;
            if (this.game.Data.ShrowdOn & !this.game.Data.ShrowdPeek)
            {
              num70 = 0;
              if (this.game.Data.Round > 0 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].get_SeeNow(Index) > 0 | this.game.Data.Turn == Index & (this.game.Data.MapObj[0].HexObj[x, y].MaxRecon > 0 | !this.game.Data.ShrowdOn))
                num70 = 1;
            }
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].get_SeeNow(Index) < 1 | this.game.Data.Turn == Index & this.game.Data.MapObj[0].HexObj[x, y].MaxRecon < 1 & this.game.Data.ShrowdOn && this.game.Data.ShrowdPeek &  this.game.Data.RuleVar[874] == 1.0)
              num70 = 0;
            if (this.game.Data.MapObj[0].HexObj[x, y].MaxRecon < 1 & this.game.Data.ShrowdOn)
              num70 = 0;
            if (num70 == 1)
            {
              if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP >=  this.game.Data.RuleVar[149] &  this.game.Data.RuleVar[149] != 0.0)
              {
                num71: i32;
                num72: i32;
                num73: i32;
                num74: i32;
                if (!realhex)
                {
                  num71 = (int) Math.Round( (Conversion.Int(num12 *  index31) +  num6));
                  num72 = (int) Math.Round( (Conversion.Int(num13 *  index32) - d2 / 2f +  num7));
                  if ((index31 + 10) % 2 > 0)
                    num72 = (int) Math.Round( ( num72 + d2 / 2f));
                  num73 = (int) Math.Round( num12);
                  num74 = (int) Math.Round( num13);
                }
                else
                {
                  num71 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  index31 +  num6)));
                  num72 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  index32) - Math.Floor( d2 / 2.0) +  num7));
                  if ((index31 + 10) % 2 > 0)
                    num72 = (int) Math.Round( num72 + Math.Floor( d2 / 2.0));
                  num73 = (int) Math.Round(Math.Floor( num12));
                  num74 = (int) Math.Round(Math.Floor( num13));
                }
                let mut num75: i32 =  (int) Math.Round( num71 -  num73 / 2.0);
                let mut num76: i32 =  (int) Math.Round( num72 -  num74 / 2.0);
                num60 = num73 * 2;
                let mut num77: i32 =  num74 * 2;
                if (num76 < 1)
                  num76 = 1;
                if ( num75 -  sizeF.Width / 2.0 < 2.0)
                  num75 = (int) Math.Round( ( num75 + sizeF.Width / 2f));
                if ( num75 +  sizeF.Width / 2.0 >  bwidth)
                  num75 = (int) Math.Round( ( num75 - sizeF.Width / 2f));
                str8: String = Strings.UCase(this.game.HandyFunctionsObj.GetHexName(x, y, this.game.EditObj.MapSelected));
                sizeF =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP <  this.game.Data.RuleVar[148] ? Expression.MeasureString(str8, this.game.MarcFont11b) : Expression.MeasureString(str8, this.game.MarcFont7);
                let mut num78: i32 =  (int) Math.Round( ( num75 - sizeF.Width / 2f));
                if ( num78 +  sizeF.Width >  tempBmp.Width)
                  num78 = (int) Math.Round( ( num78 - ( num78 + sizeF.Width -  tempBmp.Width)));
                if (num78 < 0)
                  num78 += -num78;
                if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP >=  this.game.Data.RuleVar[148])
                {
                  DrawMod.DrawBlock(ref Expression, num78, (int) Math.Round( num76 +  num77 / 2.0), (int) Math.Round( sizeF.Width), (int) Math.Round( sizeF.Height), 0, 0, 0, 128);
                  DrawMod.DrawTextColouredMarc(ref Expression, str8, this.game.MarcFont7, num78, (int) Math.Round( num76 +  num77 / 2.0), Color.White);
                }
                else
                {
                  DrawMod.DrawBlock(ref Expression, num78, (int) Math.Round( num76 +  num77 / 2.0), (int) Math.Round( sizeF.Width), (int) Math.Round( sizeF.Height), 0, 0, 0, 128);
                  DrawMod.DrawTextColouredMarc(ref Expression, str8, this.game.MarcFont11b, num78, (int) Math.Round( num76 +  num77 / 2.0), Color.White);
                }
              }
              else
              {
                let mut num79: i32 =   this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].VP >=  this.game.Data.RuleVar[149] &  this.game.Data.RuleVar[149] != 0.0 ? 1 : 0;
              }
            }
          }
        }
        Rectangle[] rectangleArray = Rectangle::new[10000];
        let mut index33: i32 =  -1;
        let mut num80: i32 =  (int) Math.Round( num6 / 2.0);
        if (alsoHQ)
        {
          let mut num81: i32 =  1;
          do
          {
            let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
            for (let mut nr: i32 =  0; nr <= regimeCounter2; nr += 1)
            {
              if (num81 == 1 & this.game.HandyFunctionsObj.GetRegime(nr) == this.game.HandyFunctionsObj.GetRegime(index1) | num81 == 2 & this.game.HandyFunctionsObj.GetRegime(nr) != this.game.HandyFunctionsObj.GetRegime(index1))
              {
                let mut num82: i32 =  5;
                do
                {
                  let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                  for (let mut index34: i32 =  0; index34 <= unitCounter; index34 += 1)
                  {
                    if (this.game.Data.UnitObj[index34].Historical > -1 & this.game.Data.UnitObj[index34].Regime == nr && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Type == num82 && this.game.Data.UnitObj[index34].IsHQ)
                    {
                      coordinate = this.game.HandyFunctionsObj.GetAverageHQCoordinate(index34);
                      if (coordinate.onmap)
                      {
                        let mut x: i32 =  coordinate.x;
                        let mut y: i32 =  coordinate.y;
                        let mut num83: i32 =  x;
                        let mut num84: i32 =  y;
                        if (this.game.EditObj.MiniMapOffset > 0)
                        {
                          num83 -= this.game.EditObj.MiniMapOffset;
                          if (num83 < 0)
                            num83 += this.game.Data.MapObj[0].MapWidth;
                        }
                        num85: i32;
                        num86: i32;
                        num87: i32;
                        num88: i32;
                        if (!realhex)
                        {
                          num85 = (int) Math.Round( (Conversion.Int(num12 *  num83) +  num6));
                          num86 = (int) Math.Round( (Conversion.Int(num13 *  num84) - d2 / 2f +  num7));
                          if ((num83 + 10) % 2 > 0)
                            num86 = (int) Math.Round( ( num86 + d2 / 2f));
                          num87 = (int) Math.Round( num12);
                          num88 = (int) Math.Round( num13);
                        }
                        else
                        {
                          num85 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num12) *  num83 +  num6)));
                          num86 = (int) Math.Round(Conversion.Int(Math.Floor(Math.Floor( num13) *  num84) - Math.Floor( d2 / 2.0) +  num7));
                          if ((num83 + 10) % 2 > 0)
                            num86 = (int) Math.Round( num86 + Math.Floor( d2 / 2.0));
                          num87 = (int) Math.Round(Math.Floor( num12));
                          num88 = (int) Math.Round(Math.Floor( num13));
                        }
                        str9: String = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].CounterString;
                        if (str9.Length > 6)
                          str9 = Strings.Left(str9, 6);
                        sizeF = this.game.ScreenHeight <= 800 ? (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Type > 5 ? Expression.MeasureString(str9, this.game.MarcFont4) : Expression.MeasureString(str9, this.game.MarcFont10)) : (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Type > 5 ? Expression.MeasureString(str9, this.game.MarcFont3) : Expression.MeasureString(str9, this.game.MarcFont4));
                        let mut num89: i32 =  0;
                        let mut num90: i32 =  0;
                        let mut num91: i32 =  1;
                        let mut num92: i32 =  num85;
                        let mut num93: i32 =  num86;
                        let mut num94: i32 =  0;
                        let mut num95: i32 =  0;
                        let mut num96: i32 =  0;
                        let mut num97: i32 =  0;
                        let mut index35: i32 =  index33 + 1;
                        rectangleArray[index35] = Rectangle::new(-100, -100, this.game.EditObj.MiniMap.Width + 200, 100);
                        let mut index36: i32 =  index35 + 1;
                        rectangleArray[index36] = Rectangle::new(-100, this.game.EditObj.MiniMap.Height, this.game.EditObj.MiniMap.Width + 200 + num80, 100);
                        let mut index37: i32 =  index36 + 1;
                        rectangleArray[index37] = Rectangle::new(-100, -100, 100, this.game.EditObj.MiniMap.Height + 100);
                        let mut index38: i32 =  index37 + 1;
                        rectangleArray[index38] = Rectangle::new(this.game.EditObj.MiniMap.Width - num80, -100, 100 + num80, this.game.EditObj.MiniMap.Height + 100);
                        index33 = index38 + 1;
                        while (num89 == 0)
                        {
                          rectangleArray[index33] = Rectangle::new((int) Math.Round( num85 +  num87 / 2.0 + 1.0 -  sizeF.Width / 2.0), num86 - 2 - 0, (int) Math.Round( (sizeF.Width - 2f)), (int) Math.Round( (sizeF.Height - 2f)));
                          num89 = -1;
                          let mut num98: i32 =  index33 - 1;
                          for (let mut index39: i32 =  0; index39 <= num98; index39 += 1)
                          {
                            if (rectangleArray[index39].IntersectsWith(rectangleArray[index33]))
                            {
                              num89 = 0;
                              num90 += 1;
                              if (num90 > 4)
                              {
                                num90 = 1;
                                num91 += 1;
                              }
                              if (num90 == 1)
                              {
                                if (num94 == 0 & this.game.HandyFunctionsObj.GetRegime(this.game.Data.MapObj[0].HexObj[Math.Max(0, this.game.Data.UnitObj[index34].X - num91 * 4), this.game.Data.UnitObj[index34].Y].Regime) == this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index34].Regime) | this.game.Data.MapObj[0].HexObj[Math.Max(0, this.game.Data.UnitObj[index34].X - num91 * 4), this.game.Data.UnitObj[index34].Y].Regime == -1)
                                {
                                  num85 = num92 - 4 * num91;
                                }
                                else
                                {
                                  num85 = num92 - (int) Math.Round(0.5 *  num91);
                                  num94 = 1;
                                }
                              }
                              if (num90 == 2)
                              {
                                if (num95 == 0 & this.game.HandyFunctionsObj.GetRegime(this.game.Data.MapObj[0].HexObj[Math.Min(this.game.Data.MapObj[0].MapWidth, this.game.Data.UnitObj[index34].X + num91 * 4), this.game.Data.UnitObj[index34].Y].Regime) == this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index34].Regime) | this.game.Data.MapObj[0].HexObj[Math.Min(this.game.Data.MapObj[0].MapWidth, this.game.Data.UnitObj[index34].X + num91 * 4), this.game.Data.UnitObj[index34].Y].Regime == -1)
                                {
                                  num85 = num92 + 4 * num91;
                                }
                                else
                                {
                                  num85 = num92 + (int) Math.Round(0.5 *  num91);
                                  num95 = 1;
                                }
                              }
                              if (num90 == 3)
                              {
                                if (num96 == 0 & this.game.HandyFunctionsObj.GetRegime(this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index34].X, Math.Min(this.game.Data.MapObj[0].MapHeight, this.game.Data.UnitObj[index34].Y + num91 * 4)].Regime) == this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index34].Regime) | this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index34].X, Math.Min(this.game.Data.MapObj[0].MapHeight, this.game.Data.UnitObj[index34].Y + num91 * 4)].Regime == -1)
                                {
                                  num86 = num93 + 4 * num91;
                                }
                                else
                                {
                                  num86 = num93 + (int) Math.Round(0.5 *  num91);
                                  num96 = 1;
                                }
                              }
                              if (num90 == 4)
                              {
                                if (num97 == 0 & this.game.HandyFunctionsObj.GetRegime(this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index34].X, Math.Max(0, this.game.Data.UnitObj[index34].Y - num91 * 4)].Regime) == this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index34].Regime) | this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index34].X, Math.Max(0, this.game.Data.UnitObj[index34].Y - num91 * 4)].Regime == -1)
                                {
                                  num86 = num93 - 4 * num91;
                                  break;
                                }
                                num86 = num93 - (int) Math.Round(0.5 *  num91);
                                num97 = 1;
                                break;
                              }
                              break;
                            }
                          }
                          if (num91 > 49)
                            goto label_724;
                        }
                        let mut num99: i32 =  0;
                        let mut num100: i32 =  0;
                        let mut num101: i32 =  0;
                        if (this.game.Data.UnitObj[index34].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
                        {
                          num99 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Red;
                          num100 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Green;
                          num101 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Blue;
                        }
                        let mut num102: i32 =  num99 + this.game.Data.RegimeObj[nr].Red;
                        let mut num103: i32 =  num100 + this.game.Data.RegimeObj[nr].Green;
                        let mut num104: i32 =  num101 + this.game.Data.RegimeObj[nr].Blue;
                        let mut integer: i32 =  Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(index34));
                        if (this.game.Data.PeopleObj[integer].Red > -1)
                        {
                          num102 = this.game.Data.PeopleObj[integer].Red;
                          num103 = this.game.Data.PeopleObj[integer].Green;
                          num104 = this.game.Data.PeopleObj[integer].Blue;
                        }
                        let mut a: i32 =  (int) byte.MaxValue;
                        if (this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index34].Regime) != this.game.HandyFunctionsObj.GetRegime(index1))
                          a = 128;
                        DrawMod.DrawBlock(ref Expression, (int) Math.Round( num85 +  num87 / 2.0 - 0.0 -  sizeF.Width / 2.0), num86 - 3 - 0, (int) Math.Round( (sizeF.Width + 0.0f)), (int) Math.Round( (sizeF.Height + 0.0f)), (int) Math.Round( num102 / 2.5), (int) Math.Round( num103 / 2.5), (int) Math.Round( num104 / 2.5), a);
                        if (this.game.ScreenHeight > 800)
                        {
                          if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Type <= 5)
                            DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont4, (int) Math.Round( num85 +  num87 / 2.0 -  sizeF.Width / 2.0), num86 - 2, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                          else
                            DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont3, (int) Math.Round( num85 +  num87 / 2.0 -  sizeF.Width / 2.0), num86 - 2, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                        }
                        else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index34].Historical].Type <= 5)
                          DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont10, (int) Math.Round( num85 +  num87 / 2.0 -  sizeF.Width / 2.0), num86 - 2, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                        else
                          DrawMod.DrawTextColouredMarc(ref Expression, str9, this.game.MarcFont4, (int) Math.Round( num85 +  num87 / 2.0 -  sizeF.Width / 2.0), num86 - 2, Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                        DrawMod.DrawRectangle(ref Expression, (int) Math.Round( num85 +  num87 / 2.0 -  sizeF.Width / 2.0 - 0.0), num86 - 3 - 0, (int) Math.Round( (sizeF.Width + 0.0f)), (int) Math.Round( (sizeF.Height + 0.0f)), (int) Math.Round( num102 / 1.5), (int) Math.Round( num103 / 1.5), (int) Math.Round( num104 / 1.5), a);
                        index33 += 1;
                      }
                    }
                  }
label_724:
                  num82 += 1;
                }
                while (num82 <= 8);
              }
            }
            num81 += 1;
          }
          while (num81 <= 1);
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub DrawSFTypeGraphic: Bitmap(
      sfTypeNr: i32,
      bool isMilitia,
      cultureGroupId: i32,
      regimeNr: i32,
      fromUnr: i32)
    {
      bool flag1 = false;
      if (fromUnr > -1 & fromUnr <= this.game.Data.UnitCounter && this.game.Data.UnitObj[fromUnr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[fromUnr].Historical].TempVar1 == 1)
        flag1 = true;
      if (this.game.Data.SFTypeObj[sfTypeNr].Theater == 2)
        flag1 = true;
      let mut stringListById: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 441, 0, 0));
      let mut num1: i32 =  this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[89];
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[61] < 1 && this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[81] > 0)
        cultureGroupId = 105;
      let mut num4: i32 =  Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Red + 30);
      let mut num5: i32 =  Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Green + 30);
      let mut num6: i32 =  Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Blue + 30);
      let mut num7: i32 =  (int) Math.Round( (num4 + num4 + 180) / 3.0);
      let mut num8: i32 =  (int) Math.Round( (num5 + num5 + 180) / 3.0);
      let mut num9: i32 =  (int) Math.Round( (num6 + num6 + 180) / 3.0);
      if (flag1)
      {
        num4 = Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Red2 + 30);
        num5 = Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Green2 + 30);
        num6 = Math.Min((int) byte.MaxValue, this.game.Data.RegimeObj[regimeNr].Blue2 + 30);
        num7 = (int) Math.Round( (num4 + 180 + 180) / 3.0);
        num8 = (int) Math.Round( (num5 + 180 + 180) / 3.0);
        num9 = (int) Math.Round( (num6 + 180 + 180) / 3.0);
      }
      let mut num10: i32 =  99;
      let mut num11: i32 =  99;
      let mut length1: i32 =  this.game.Data.StringListObj[stringListById].Length;
      for (let mut index1: i32 =  0; index1 <= length1; index1 += 1)
      {
        if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 0])) == num1 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 10])) > -1)
        {
          bool flag2 = true;
          let mut num12: i32 =  0;
          do
          {
            let mut num13: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 3 + num12 * 4]));
            if (num13 > 0)
            {
              let mut index2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 2 + num12 * 4]));
              let mut num14: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 4 + num12 * 4]));
              let mut num15: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 5 + num12 * 4]));
              switch (num13)
              {
                case 1:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index2] != num14)
                  {
                    flag2 = false;
                    break;
                  }
                  break;
                case 2:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index2] < num14 | this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index2] > num15)
                  {
                    flag2 = false;
                    break;
                  }
                  break;
                case 3:
                  if (cultureGroupId != num14)
                  {
                    flag2 = false;
                    break;
                  }
                  break;
              }
            }
            if (flag2)
              num12 += 1;
            else
              break;
          }
          while (num12 <= 1);
          if (flag2)
          {
            let mut num16: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 13]));
            let mut num17: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 14]));
            let mut num18: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 16]));
            let mut num19: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index1, 17]));
            if (num16 > 0 & num17 > 0)
            {
              if (num16 > num2)
                num2 = num16;
              if (num17 > num3)
                num3 = num17;
              if (num18 + num19 < num10 + num11)
              {
                num10 = num18;
                num11 = num19;
              }
            }
          }
        }
      }
      if (!(num2 > 0 & num3 > 0))
        return (Bitmap) null;
      let mut width: i32 =  num2 - (num10 + num11);
      let mut height: i32 =  num3;
      bitmap1: Bitmap = new Bitmap(width, height, PixelFormat.Format32bppPArgb);
      Graphics graphics = Graphics.FromImage((Image) bitmap1);
      graphics.Clear(Color.Transparent);
      let mut length2: i32 =  this.game.Data.StringListObj[stringListById].Length;
      for (let mut index3: i32 =  0; index3 <= length2; index3 += 1)
      {
        if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 0])) == num1)
        {
          bool flag3 = true;
          let mut num20: i32 =  0;
          do
          {
            let mut num21: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 3 + num20 * 4]));
            if (num21 > 0)
            {
              let mut index4: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 2 + num20 * 4]));
              let mut num22: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 4 + num20 * 4]));
              let mut num23: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 5 + num20 * 4]));
              switch (num21)
              {
                case 1:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index4] != num22)
                  {
                    flag3 = false;
                    break;
                  }
                  break;
                case 2:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index4] < num22 | this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index4] > num23)
                  {
                    flag3 = false;
                    break;
                  }
                  break;
                case 3:
                  if (cultureGroupId != num22)
                  {
                    flag3 = false;
                    break;
                  }
                  break;
                case 4:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index4] < num22)
                  {
                    flag3 = false;
                    break;
                  }
                  break;
                case 5:
                  if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[index4] > num22)
                  {
                    flag3 = false;
                    break;
                  }
                  break;
              }
            }
            if (flag3)
              num20 += 1;
            else
              break;
          }
          while (num20 <= 1);
          if (flag3)
          {
            let mut index5: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 10]));
            let mut num24: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 11]));
            let mut num25: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 12]));
            let mut num26: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index3, 15]));
            let mut num27: i32 =  1 + num24 * (num2 + 1);
            let mut y: i32 =  1 + num25 * (num3 + 1);
            Rectangle rectangle1;
            Rectangle rectangle2;
            switch (num26)
            {
              case 1:
                if (isMilitia)
                {
                  ref Graphics local1 = ref graphics;
                  bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index5]);
                  ref local2: Bitmap = ref bitmap2;
                  rectangle2 = Rectangle::new(num27 + num10, y, width, height);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(0, 0, width, height);
                  let mut destrect: &Rectangle = &rectangle1
                  double r =  ( num7 / 256f);
                  double g =  ( num8 / 256f);
                  double b =  ( num9 / 256f);
                  DrawMod.DrawSimplePart2ColouredNew(ref local1, ref local2, srcrect, destrect,  r,  g,  b, 0.25f);
                  continue;
                }
                ref Graphics local3 = ref graphics;
                bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index5]);
                ref local4: Bitmap = ref bitmap3;
                rectangle2 = Rectangle::new(num27 + num10, y, width, height);
                let mut srcrect1: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(0, 0, width, height);
                let mut destrect1: &Rectangle = &rectangle1
                double r1 =  ( num7 / 256f);
                double g1 =  ( num8 / 256f);
                double b1 =  ( num9 / 256f);
                DrawMod.DrawSimplePart2ColouredNew(ref local3, ref local4, srcrect1, destrect1,  r1,  g1,  b1, 1f);
                continue;
              case 2:
                if (isMilitia)
                {
                  ref Graphics local5 = ref graphics;
                  bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index5]);
                  ref local6: Bitmap = ref bitmap4;
                  rectangle1 = Rectangle::new(num27 + num10, y, width, height);
                  let mut srcrect2: &Rectangle = &rectangle1
                  rectangle2 = Rectangle::new(0, 0, width, height);
                  let mut destrect2: &Rectangle = &rectangle2
                  double r2 =  ( num4 / 256f);
                  double g2 =  ( num5 / 256f);
                  double b2 =  ( num6 / 256f);
                  DrawMod.DrawSimplePart2ColouredNew(ref local5, ref local6, srcrect2, destrect2,  r2,  g2,  b2, 0.25f);
                  continue;
                }
                ref Graphics local7 = ref graphics;
                bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index5]);
                ref local8: Bitmap = ref bitmap5;
                rectangle2 = Rectangle::new(num27 + num10, y, width, height);
                let mut srcrect3: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(0, 0, width, height);
                let mut destrect3: &Rectangle = &rectangle1
                double r3 =  ( num4 / 256f);
                double g3 =  ( num5 / 256f);
                double b3 =  ( num6 / 256f);
                DrawMod.DrawSimplePart2ColouredNew(ref local7, ref local8, srcrect3, destrect3,  r3,  g3,  b3, 1f);
                continue;
              default:
                ref Graphics local9 = ref graphics;
                bitmap6: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.Data.EventPicNr[index5]);
                ref local10: Bitmap = ref bitmap6;
                rectangle2 = Rectangle::new(num27 + num10, y, width, height);
                let mut srcrect4: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(0, 0, width, height);
                let mut destrect4: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect4, destrect4);
                continue;
            }
          }
        }
      }
      graphics.Dispose();
      return bitmap1;
    }

    pub DrawUnitSmall: Bitmap(
      nr: i32,
      bool forcehighlight = false,
      Graphics toG = null,
      let mut tx: i32 =  0,
      let mut ty: i32 =  0,
      bool ShowAttacker = false,
      let mut OverruleHis: i32 =  -1,
      let mut OverrulePower: i32 =  -1,
      let mut OverruleRegime: i32 =  -1,
      bool FullRecon = false)
    {
      Coordinate coordinate = Coordinate::new();
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = SizeF::new();
      int[] numArray1 = new int[1];
      int[] numArray2 = this.game.Data.SFTypeCounter >= 101 ? new int[this.game.Data.SFTypeCounter + 1] : new int[101];
      if ( this.game.Data.RuleVar[344] == 1.0 & this.game.EditObj.HideUnit == 0)
        this.game.EditObj.HideUnit = 2;
      if (!Information.IsNothing( toG))
      {
        this.g2 = toG;
      }
      else
      {
        this.g2 = Graphics.FromImage((Image) this.tmpbmp2b);
        this.g2.Clear(Color.FromArgb(0, 0, 0, 0));
      }
      bool isMilitia = OverruleHis != -1 ? this.game.Data.HistoricalUnitObj[OverruleHis].GiveHisVarValue(11) > 0 : this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].GiveHisVarValue(11) > 0;
      index1: i32;
      bool flag1;
      regime1: i32;
      integer1: i32;
      index2: i32;
      bitmap1: Bitmap;
      float num1;
      float num2;
      float num3;
      if (OverruleHis == -1)
      {
        if (this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn | this.game.Data.Round < 1 | !this.game.Data.FOWOn)
          coordinate.x = 3;
        else
          coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(nr, this.game.Data.Turn);
        index1 = !this.game.Data.UnitObj[nr].IsHQ ? this.game.Data.UnitObj[nr].HQ : nr;
        flag1 = !(this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn & this.game.Data.Round > 0) || this.game.Data.UnitObj[nr].X > -1 && this.game.HandyFunctionsObj.CanUnitMove2(nr);
        bool flag2 = false;
        if (Information.IsNothing( this.game.EditObj.TempUnitList))
          this.game.EditObj.TempUnitList = UnitList::new();
        bool flag3;
        if (this.game.EditObj.OrderType == 2 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 12 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 11 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 13 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 14 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 15 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
        {
          flag2 = false;
          flag3 = true;
        }
        if (this.game.EditObj.OrderType == 9)
        {
          if (this.game.EditObj.OrderUnit == nr)
            flag2 = true;
          if (this.game.EditObj.OrderTarget == nr)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 3 && this.game.EditObj.OrderUnit == nr)
          flag2 = true;
        if (this.game.EditObj.OrderType == 19)
        {
          if (this.game.EditObj.OrderUnit == nr)
            flag2 = true;
          if (this.game.EditObj.OrderTarget == nr)
            flag2 = true;
        }
        if (this.game.EditObj.OrderType == 33 && this.game.EditObj.OrderUnit == nr)
          flag2 = true;
        if (forcehighlight && this.game.EditObj.UnitSelected == nr)
          flag2 = true;
        if (this.game.EditObj.LayerSupplyOn && this.game.EditObj.LayerSupplyHQ == this.game.Data.UnitObj[nr].HQ)
          flag2 = true;
        regime1 = this.game.Data.UnitObj[nr].Regime;
        integer1 = Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(nr));
        index2 = regime1;
        if (this.game.Data.PeopleObj[integer1].RegCol > -1)
          index2 = this.game.Data.PeopleObj[integer1].RegCol;
        if (Information.IsNothing( this.game.Data.RegimeObj[index2].TempCountersmall))
          this.game.Data.RegimeObj[index2].DoTempCounterSmall();
        if (OverruleRegime > -1 && Information.IsNothing( this.game.Data.RegimeObj[OverruleRegime].TempCountersmall))
          this.game.Data.RegimeObj[OverruleRegime].DoTempCounterSmall();
        if (this.game.Data.UnitObj[nr].Regime == -1)
        {
          ref Graphics local1 = ref this.g2;
          bitmap1 = BitmapStore.GetBitmap(this.game.DEFAULTCOUNTER);
          ref local2: Bitmap = ref bitmap1;
          let mut x: i32 =  tx;
          let mut y: i32 =  ty;
          DrawMod.DrawSimple(ref local1, ref local2, x, y);
        }
        else
        {
          let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType;
          red: i32;
          green: i32;
          blue: i32;
          if (this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
          {
            red = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Red;
            green = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Green;
            blue = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Blue;
          }
          if (red != 0 | green != 0 | blue != 0 | isMilitia)
          {
            if (!flag2)
            {
              float num4 =  ((int) byte.MaxValue + red) / 256f;
              float num5 =  ((int) byte.MaxValue + green) / 256f;
              float num6 =  ((int) byte.MaxValue + blue) / 256f;
              if ( num4 > 1.0)
                num4 = 1f;
              if ( num5 > 1.0)
                num5 = 1f;
              if ( num6 > 1.0)
                num6 = 1f;
              if (0.0 >  num4)
                num1 = 0.0f;
              if (0.0 >  num5)
                num2 = 0.0f;
              if (0.0 >  num6)
                num3 = 0.0f;
              if (isMilitia)
                DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmallHigh, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
              else
                DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmall, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx, ty, 19, 19, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 18, 18, 0, 0, 0, (int) byte.MaxValue);
            }
            else
            {
              float num7 =  ((int) byte.MaxValue + red) / 256f;
              float num8 =  ((int) byte.MaxValue + green) / 256f;
              float num9 =  ((int) byte.MaxValue + blue) / 256f;
              if ( num7 > 1.0)
                num7 = 1f;
              if ( num8 > 1.0)
                num8 = 1f;
              if ( num9 > 1.0)
                num9 = 1f;
              if (0.0 >  num7)
                num1 = 0.0f;
              if (0.0 >  num8)
                num2 = 0.0f;
              if (0.0 >  num9)
                num3 = 0.0f;
              if (isMilitia)
                DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmallHigh, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
              else
                DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmall, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx, ty, 19, 19, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 18, 18, 0, 0, 0, (int) byte.MaxValue);
            }
          }
          else if (!flag2)
          {
            DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmall, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
            if (flag3 & ShowAttacker)
              DrawMod.DrawRectangle(ref this.g2, tx, ty, 19, 19, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
            if (flag3 & ShowAttacker)
              DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 18, 18, 0, 0, 0, (int) byte.MaxValue);
          }
          else
          {
            DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCountersmallHigh, Rectangle::new(19 * landscapeType, 0, 19, 19), Rectangle::new(tx, ty, 19, 19));
            if (flag3 & ShowAttacker)
              DrawMod.DrawRectangle(ref this.g2, tx, ty, 19, 19, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
            if (flag3 & ShowAttacker)
              DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 18, 18, 0, 0, 0, (int) byte.MaxValue);
          }
        }
      }
      else
      {
        let mut num10: i32 =  OverruleHis > -1 & OverruleRegime > -1 ? 1 : 0;
      }
      if (index1 > -1 & !this.game.Data.UnitObj[nr].IsHQ)
      {
        if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
        {
          let mut red: i32 =  this.game.Data.UnitObj[index1].Red;
          let mut green: i32 =  this.game.Data.UnitObj[index1].Green;
          let mut blue: i32 =  this.game.Data.UnitObj[index1].Blue;
          DrawMod.DrawBlockGradient2(ref this.g2, tx, ty + 12, 18, 6, Color.FromArgb(0, red, green, blue), Color.FromArgb(235, red, green, blue));
        }
      }
      else if (this.game.Data.UnitObj[nr].IsHQ & this.game.Data.UnitObj[nr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Type < 8)
      {
        let mut red: i32 =  this.game.Data.UnitObj[nr].Red;
        let mut green: i32 =  this.game.Data.UnitObj[nr].Green;
        let mut blue: i32 =  this.game.Data.UnitObj[nr].Blue;
        DrawMod.DrawBlockGradient2(ref this.g2, tx, ty + 1, 18, 17, Color.FromArgb(0, red, green, blue), Color.FromArgb(205, red, green, blue));
      }
      if (OverruleHis == -1)
      {
        let mut num11: i32 =  0;
        if (this.game.Data.UnitObj[nr].HQ == -1 & !this.game.Data.UnitObj[nr].IsHQ)
          num11 = 0;
        if (!this.game.EditObj.PrefMinimalistCounter &&  this.game.Data.RuleVar[847] == 1.0)
        {
          ref Graphics local3 = ref this.g2;
          bitmap1 = BitmapStore.GetBitmap(this.game.Data.RegimeObj[index2].NationalIconSprite);
          ref local4: Bitmap = ref bitmap1;
          let mut x: i32 =  tx;
          let mut y: i32 =  ty;
          DrawMod.DrawScaled(ref local3, ref local4, x, y, 4, 4);
        }
        let mut num12: i32 =  -1;
        let mut num13: i32 =  -1;
        sfTypeNr: i32;
        num14: i32;
        if (coordinate.x > 1)
        {
          if (!this.game.Data.UnitObj[nr].IsHQ)
          {
            let mut sfCount1: i32 =  this.game.Data.UnitObj[nr].SFCount;
            nr1: i32;
            for (let mut index3: i32 =  0; index3 <= sfCount1; index3 += 1)
            {
              nr1 = this.game.Data.UnitObj[nr].SFList[index3];
              let mut index4: i32 =  this.game.Data.SFObj[nr1].Type;
              let mut qty: i32 =  this.game.Data.SFObj[nr1].Qty;
              if (this.game.Data.UnitObj[nr].X > -1 && index4 > -1 && this.game.Data.SFTypeObj[index4].Theater != 1 & this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType].IsSea)
                index4 = -1;
              if (index4 > -1)
              {
                let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[index4].SymbolGroup;
                let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[index4].SymbolWeight;
                if (symbolGroup > -1)
                {
                  numArray2[symbolGroup] = numArray2[symbolGroup] + symbolWeight * qty * 10;
                  if (this.game.Data.SFTypeObj[index4].CarryCap > 0)
                    numArray2[symbolGroup] = numArray2[symbolGroup] + 1;
                  if (numArray2[symbolGroup] > num13)
                  {
                    num13 = numArray2[symbolGroup];
                    sfTypeNr = symbolGroup;
                  }
                }
              }
            }
            if (sfTypeNr > -1 | this.game.Data.UnitObj[nr].Historical > -1 & this.game.EditObj.HideUnit == 2 |  this.game.Data.RuleVar[344] == 0.0)
            {
              let mut num15: i32 =  sfTypeNr;
              sfTypeNr = -1;
              let mut index5: i32 =  -1;
              let mut num16: i32 =  0;
              let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
              for (let mut index6: i32 =  0; index6 <= sfTypeCounter; index6 += 1)
                numArray2[index6] = 0;
              let mut sfCount2: i32 =  this.game.Data.UnitObj[nr].SFCount;
              for (let mut index7: i32 =  0; index7 <= sfCount2; index7 += 1)
              {
                nr1 = this.game.Data.UnitObj[nr].SFList[index7];
                let mut type: i32 =  this.game.Data.SFObj[nr1].Type;
                let mut qty: i32 =  this.game.Data.SFObj[nr1].Qty;
                let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[type].SymbolGroup;
                let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[type].SymbolWeight;
                if (symbolGroup == num15)
                {
                  numArray2[type] = numArray2[type] + symbolWeight * qty * 10;
                  if (this.game.Data.SFTypeObj[type].CarryCap > 0)
                    numArray2[type] = numArray2[type] + 1;
                  if (numArray2[type] > num16)
                  {
                    num16 = numArray2[type];
                    sfTypeNr = type;
                    index5 = nr1;
                  }
                }
              }
              if (sfTypeNr > -1)
              {
                if ((0 & (this.game.Data.UnitObj[nr].Historical > -1 ? 1 : 0) & (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[89] > 0 ? 1 : 0)) != 0)
                {
                  let mut tv0: i32 =  this.game.Data.PeopleObj[this.game.Data.SFObj[index5].People].tv0;
                  if (this.slotCulture < 0)
                    this.slotCulture = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
                  let mut integer2: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.slotCulture].GetData(0, tv0, 1));
                  objBitmap: Bitmap = this.DrawSFTypeGraphic(sfTypeNr, isMilitia, integer2, regime1, nr);
                  if (!Information.IsNothing( objBitmap))
                  {
                    let mut num17: i32 =  0;
                    let mut num18: i32 =  0;
                    let mut w: i32 =  18;
                    let mut h: i32 =  18;
                    let mut width: i32 =  objBitmap.Width;
                    let mut height: i32 =  objBitmap.Height;
                    if (width > w | height > h)
                    {
                      if ( width /  w >  height /  h)
                      {
                        float num19 =  w /  width;
                        let mut num20: i32 =  (int) Math.Round( ( h -  height * num19));
                        num18 += (int) Math.Round( num20 / 2.0);
                        h -= num20;
                      }
                      else
                      {
                        float num21 =  h /  height;
                        let mut num22: i32 =  (int) Math.Round( ( w -  width * num21));
                        num17 += (int) Math.Round( num22 / 2.0);
                        w -= num22;
                      }
                      DrawMod.DrawScaled(ref this.g2, ref objBitmap, tx + num17, ty + num18, w, h);
                    }
                    else
                      DrawMod.DrawSimple(ref this.g2, ref objBitmap, tx + num17 + (int) Math.Round( (w - width) / 2.0), ty + num18 + (int) Math.Round( (h - height) / 2.0));
                    objBitmap.Dispose();
                  }
                }
                else if (sfTypeNr > -1)
                {
                  if (sfTypeNr > -1)
                    nr1 = this.game.Data.SFTypeObj[sfTypeNr].SymbolSpriteID;
                  let mut nr2: i32 =  -1;
                  if (regime1 > -1 & sfTypeNr > -1)
                  {
                    if (this.game.Data.RegimeObj[regime1].ExtraGraphicUse > -1)
                    {
                      let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                      for (let mut index8: i32 =  0; index8 <= extraCounter; index8 += 1)
                      {
                        if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index8] == this.game.Data.RegimeObj[regime1].ExtraGraphicUse)
                          nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index8];
                      }
                    }
                    else if (this.game.Data.PeopleObj[integer1].ExtraGraphicUse > -1)
                    {
                      let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                      for (let mut index9: i32 =  0; index9 <= extraCounter; index9 += 1)
                      {
                        if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index9] == this.game.Data.PeopleObj[integer1].ExtraGraphicUse)
                          nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index9];
                      }
                    }
                  }
                  bool flag4;
                  if (this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
                  {
                    nr1 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx <= -1 ? -1 : this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx];
                    if (nr1 == -1 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter > 0 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= this.game.NATO.GetUpperBound(0))
                      nr1 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter];
                    flag4 = true;
                    if (this.game.Data.UnitObj[nr].HistoricalSubPart > -1)
                    {
                      if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                        nr2 = this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                      else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                        nr2 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                    }
                  }
                  num12 = nr1;
                  if (this.game.Data.UnitObj[nr].Regime > -1 & sfTypeNr > -1)
                  {
                    if (this.game.Data.SFTypeObj[sfTypeNr].SymbolOverrule)
                    {
                      num1 = 1f;
                      num2 = 1f;
                      num3 = 1f;
                    }
                    else
                    {
                      num1 =  this.game.Data.RegimeObj[index2].Red3 /  byte.MaxValue;
                      num2 =  this.game.Data.RegimeObj[index2].Green3 /  byte.MaxValue;
                      num3 =  this.game.Data.RegimeObj[index2].Blue3 /  byte.MaxValue;
                    }
                    if (flag4)
                    {
                      num1 = 1f;
                      num2 = 1f;
                      num3 = 1f;
                    }
                    num23: i32;
                    if (this.game.EditObj.HideUnit == 2)
                    {
                      num11 = -1;
                      num23 = num14 + 1;
                    }
                    else
                      num23 = -2;
                    --tx;
                    ty += 2;
                    if (nr1 > 0)
                    {
                      ref Graphics local5 = ref this.g2;
                      bitmap1 = BitmapStore.GetBitmap(nr1, -1);
                      ref local6: Bitmap = ref bitmap1;
                      let mut x: i32 =  num11 + tx;
                      let mut y: i32 =  ty + 2 + num23;
                      DrawMod.DrawSimple(ref local5, ref local6, x, y);
                    }
                    this.g2.ResetTransform();
                    if (nr2 > -1)
                    {
                      ref Graphics local7 = ref this.g2;
                      bitmap1 = BitmapStore.GetBitmap(nr2, -1);
                      ref local8: Bitmap = ref bitmap1;
                      let mut x: i32 =  num11 + tx;
                      let mut y: i32 =  ty + num23 + 2;
                      DrawMod.DrawSimple(ref local7, ref local8, x, y);
                    }
                  }
                  else
                  {
                    if (nr1 > 0)
                    {
                      ref Graphics local9 = ref this.g2;
                      bitmap1 = BitmapStore.GetBitmap(nr1, -1);
                      ref local10: Bitmap = ref bitmap1;
                      let mut x: i32 =  num11 + tx;
                      let mut y: i32 =  ty + 2 + num14;
                      DrawMod.DrawSimple(ref local9, ref local10, x, y);
                    }
                    this.g2.ResetTransform();
                    if (nr2 > -1)
                    {
                      ref Graphics local11 = ref this.g2;
                      bitmap1 = BitmapStore.GetBitmap(nr2, -1);
                      ref local12: Bitmap = ref bitmap1;
                      let mut x: i32 =  num11 + tx;
                      let mut y: i32 =  ty + num14 + 2;
                      DrawMod.DrawSimple(ref local11, ref local12, x, y);
                    }
                  }
                }
              }
            }
          }
          else
          {
            if (this.game.EditObj.HideUnit == 2 && this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
              num14 = 8;
            if (this.game.Data.UnitObj[nr].IsHQ)
              num14 = 0;
            sfTypeNr = -1;
            let mut regime2: i32 =  this.game.Data.UnitObj[nr].Regime;
            if (this.game.Data.UnitObj[nr].Regime > -1)
            {
              let mut hqSpriteNr: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr;
              float num24 =  this.game.Data.RegimeObj[index2].Red / 256f;
              float num25 =  this.game.Data.RegimeObj[index2].Green / 256f;
              float num26 =  this.game.Data.RegimeObj[index2].Blue / 256f;
              ref Graphics local13 = ref this.g2;
              bitmap1 = BitmapStore.GetBitmap(hqSpriteNr, -1);
              ref local14: Bitmap = ref bitmap1;
              let mut x1: i32 =  tx - 2;
              let mut y1: i32 =  ty + 3;
              double r1 =  num24 - 1.0;
              double g1 =  num25 - 1.0;
              double b1 =  num26 - 1.0;
              DrawMod.Draw(ref local13, ref local14, x1, y1,  r1,  g1,  b1, 1f);
              let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr2;
              if (this.game.Data.UnitObj[nr].Historical <= -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx > -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= 0)
                ;
              float num27;
              float num28;
              float num29;
              if (!this.game.Data.RegimeObj[index2].HQSpriteOverrule)
              {
                num27 =  this.game.Data.RegimeObj[index2].Red3 / 256f;
                num28 =  this.game.Data.RegimeObj[index2].Green3 / 256f;
                num29 =  this.game.Data.RegimeObj[index2].Blue3 / 256f;
              }
              else
              {
                num27 = 1f;
                num28 = 1f;
                num29 = 1f;
              }
              ref Graphics local15 = ref this.g2;
              bitmap1 = BitmapStore.GetBitmap(hqSpriteNr2, -1);
              ref local16: Bitmap = ref bitmap1;
              let mut x2: i32 =  tx - 2;
              let mut y2: i32 =  ty + 3;
              double r2 =  num27 - 1.0;
              double g2 =  num28 - 1.0;
              double b2 =  num29 - 1.0;
              DrawMod.Draw(ref local15, ref local16, x2, y2,  r2,  g2,  b2, 1f);
            }
          }
        }
        else if (this.game.Data.UnitObj[nr].Regime > -1)
        {
          if (this.game.Data.UnitObj[nr].IsHQ)
          {
            if (!this.game.Data.RegimeObj[index2].HQSpriteOverrule)
            {
              num1 =  this.game.Data.RegimeObj[index2].Red3 / 256f;
              num2 =  this.game.Data.RegimeObj[index2].Green3 / 256f;
              num3 =  this.game.Data.RegimeObj[index2].Blue3 / 256f;
            }
            else
            {
              num1 = 1f;
              num2 = 1f;
              num3 = 1f;
            }
          }
          else
          {
            index10: i32;
            if (this.game.Data.SFTypeObj[index10].SymbolOverrule)
            {
              num1 = 1f;
              num2 = 1f;
              num3 = 1f;
            }
            else
            {
              num1 =  this.game.Data.RegimeObj[index2].Red3 /  byte.MaxValue;
              num2 =  this.game.Data.RegimeObj[index2].Green3 /  byte.MaxValue;
              num3 =  this.game.Data.RegimeObj[index2].Blue3 /  byte.MaxValue;
            }
          }
        }
        if ( this.game.Data.RuleVar[847] < 1.0)
        {
          float num30;
          float num31;
          float num32;
          if (this.game.Data.UnitObj[nr].IsHQ)
          {
            if (!this.game.Data.RegimeObj[index2].HQSpriteOverrule)
            {
              num30 =  this.game.Data.RegimeObj[index2].Red3 / 256f;
              num31 =  this.game.Data.RegimeObj[index2].Green3 / 256f;
              num32 =  this.game.Data.RegimeObj[index2].Blue3 / 256f;
            }
            else
            {
              num30 = 1f;
              num31 = 1f;
              num32 = 1f;
            }
          }
          else if (sfTypeNr > -1)
          {
            if (this.game.Data.SFTypeObj[sfTypeNr].SymbolOverrule)
            {
              num30 = 1f;
              num31 = 1f;
              num32 = 1f;
            }
            else
            {
              num30 =  this.game.Data.RegimeObj[index2].Red2 /  byte.MaxValue;
              num31 =  this.game.Data.RegimeObj[index2].Green2 /  byte.MaxValue;
              num32 =  this.game.Data.RegimeObj[index2].Blue2 /  byte.MaxValue;
            }
          }
          else
          {
            num30 =  this.game.Data.RegimeObj[index2].Red2 /  byte.MaxValue;
            num31 =  this.game.Data.RegimeObj[index2].Green2 /  byte.MaxValue;
            num32 =  this.game.Data.RegimeObj[index2].Blue2 /  byte.MaxValue;
          }
          let mut Number: i32 =  coordinate.x != 2 ? ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon, AbsPower: true)) : ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y, AbsPower: true));
          str: String = Strings.Trim(Conversion.Str( Number));
          if (coordinate.x < 2 | Number < 0)
            str = "?";
          SizeF sizeF3 = this.g2.MeasureString(str, this.game.VicFont6);
          let mut num33: i32 =  str.Length > 1 ? (str.Length > 2 ? (int) Math.Round(2.0 + (8.0 -  sizeF3.Width / 2.0)) : 4) : 7;
          if ( num30 > 1.0)
            num30 = 1f;
          if ( num31 > 1.0)
            num31 = 1f;
          if ( num32 > 1.0)
            num32 = 1f;
          if (flag1)
            DrawMod.DrawTextVic3(ref this.g2, str, this.game.VicFont6, tx + num33, ty + 9, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( (num30 *  byte.MaxValue)), (int) Math.Round( (num31 *  byte.MaxValue)), (int) Math.Round( (num32 *  byte.MaxValue))), Color.FromArgb(196, 0, 0, 0));
          else
            DrawMod.DrawTextVic3(ref this.g2, str, this.game.VicFont6, tx + num33, ty + 9, Color.FromArgb(128, (int) Math.Round( (num30 *  byte.MaxValue)), (int) Math.Round( (num31 *  byte.MaxValue)), (int) Math.Round( (num32 *  byte.MaxValue))), Color.FromArgb(196, 0, 0, 0));
        }
      }
      else
      {
        let mut num34: i32 =  OverruleHis > -1 & OverrulePower != -9999 &  this.game.Data.RuleVar[344] == 1.0 ? 1 : 0;
      }
      if (OverruleHis == -1 &  this.game.Data.RuleVar[334] == 0.0 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
      {
        if (OverruleHis == -1 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
        {
          if (this.game.EditObj.ShowSameHistorical && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[nr].HQ > -1 & this.game.Data.UnitObj[nr].HQ == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ & !this.game.Data.UnitObj[nr].IsHQ | nr == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
          {
            float red =  this.game.Data.UnitObj[index1].Red;
            float green =  this.game.Data.UnitObj[index1].Green;
            float blue =  this.game.Data.UnitObj[index1].Blue;
            DrawMod.DrawRectangle(ref this.g2, tx, ty, 17, 17, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 2);
          }
          if (this.game.EditObj.ShowUnderHQ && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.UnitObj[nr].HQ == this.game.EditObj.UnitSelected && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
          {
            float red =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Red;
            float green =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Green;
            float blue =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Blue;
            DrawMod.DrawRectangle(ref this.g2, tx, ty, 17, 17, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 2);
          }
        }
      }
      if (this.game.EditObj.UnitSelected == nr)
      {
        DrawMod.DrawRectangle(ref this.g2, tx + 0, ty + 0, 17, 17, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
        DrawMod.DrawRectangle(ref this.g2, tx - 1, ty - 1, 19, 19, 0, 0, 0, (int) byte.MaxValue);
      }
      if (OverruleHis == -1 &&  this.game.Data.RuleVar[983] > 0.0 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime && !this.game.HandyFunctionsObj.CheckIfInCorrectFrontzone(nr, this.game.Data.UnitObj[nr].RealX(ref this.game), this.game.Data.UnitObj[nr].RealY(ref this.game)))
        DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 16, 16, (int) byte.MaxValue, 0, 0, 172);
      bitmap2: Bitmap;
      return Information.IsNothing( toG) ? this.tmpbmp2 : bitmap2;
    }

    pub DrawUnitBig: Bitmap(
      nr: i32,
      bool forcehighlight = false,
      Graphics toG = null,
      let mut tx: i32 =  0,
      let mut ty: i32 =  0,
      bool ShowAttacker = false,
      let mut OverruleHis: i32 =  -1,
      let mut OverrulePower: i32 =  -1,
      let mut OverruleRegime: i32 =  -1,
      bool FullRecon = false,
      bool mostlyHidden = false)
    {
      Coordinate coordinate = Coordinate::new();
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = SizeF::new();
      int[] numArray1 = new int[1];
      int[] numArray2 = this.game.Data.SFTypeCounter >= 101 ? new int[this.game.Data.SFTypeCounter + 1] : new int[101];
      if (!Information.IsNothing( toG))
      {
        this.g2 = toG;
      }
      else
      {
        this.g2 = Graphics.FromImage((Image) this.tmpbmp2b);
        this.g2.Clear(Color.FromArgb(0, 0, 0, 0));
      }
      if (this.game.Data.Product >= 7)
        this.g2.CompositingQuality = CompositingQuality.HighSpeed;
      bool isMilitia = OverruleHis != -1 ? this.game.Data.HistoricalUnitObj[OverruleHis].GiveHisVarValue(11) > 0 : this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].GiveHisVarValue(11) > 0;
      index1: i32;
      regime1: i32;
      integer1: i32;
      index2: i32;
      bitmap1: Bitmap;
      float num1;
      float num2;
      float num3;
      Rectangle rectangle1;
      Rectangle rectangle2;
      regime2: i32;
      if (OverruleHis == -1)
      {
        if (nr != -1)
        {
          if (this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn | this.game.Data.Round < 1 | !this.game.Data.FOWOn)
            coordinate.x = 3;
          else
            coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(nr, this.game.Data.Turn);
          index1 = !this.game.Data.UnitObj[nr].IsHQ ? this.game.Data.UnitObj[nr].HQ : nr;
          bool flag1 = !(this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn & this.game.Data.Round > 0) || this.game.Data.UnitObj[nr].X > -1 && this.game.HandyFunctionsObj.CanUnitMove2(nr);
          bool flag2 = false;
          if (Information.IsNothing( this.game.EditObj.TempUnitList))
            this.game.EditObj.TempUnitList = UnitList::new();
          bool flag3;
          if (this.game.EditObj.OrderType == 2 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 12 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 11 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 13 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 14 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 15 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
          {
            flag2 = false;
            flag3 = true;
          }
          if (this.game.EditObj.OrderType == 9)
          {
            if (this.game.EditObj.OrderUnit == nr)
              flag2 = true;
            if (this.game.EditObj.OrderTarget == nr)
              flag2 = true;
          }
          if (this.game.EditObj.OrderType == 3 && this.game.EditObj.OrderUnit == nr)
            flag2 = true;
          if (this.game.EditObj.OrderType == 19)
          {
            if (this.game.EditObj.OrderUnit == nr)
              flag2 = true;
            if (this.game.EditObj.OrderTarget == nr)
              flag2 = true;
          }
          if (this.game.EditObj.OrderType == 33 && this.game.EditObj.OrderUnit == nr)
            flag2 = true;
          if (forcehighlight && this.game.EditObj.UnitSelected == nr)
            flag2 = true;
          if (this.game.EditObj.LayerSupplyOn && this.game.EditObj.LayerSupplyHQ == this.game.Data.UnitObj[nr].HQ)
            flag2 = true;
          regime1 = this.game.Data.UnitObj[nr].Regime;
          integer1 = Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(nr));
          index2 = regime1;
          if (this.game.Data.PeopleObj[integer1].RegCol > -1)
            index2 = this.game.Data.PeopleObj[integer1].RegCol;
          if (Information.IsNothing( this.game.Data.RegimeObj[index2].TempCounterBig))
            this.game.Data.RegimeObj[index2].DoTempCounterBig();
          if (OverruleRegime > -1 && Information.IsNothing( this.game.Data.RegimeObj[OverruleRegime].TempCounterBig))
            this.game.Data.RegimeObj[OverruleRegime].DoTempCounterBig();
          if (this.game.Data.UnitObj[nr].Regime == -1)
          {
            ref Graphics local1 = ref this.g2;
            bitmap1 = BitmapStore.GetBitmap(this.game.DEFAULTCOUNTERBIG);
            ref local2: Bitmap = ref bitmap1;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local1, ref local2, x, y);
          }
          else
          {
            let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType;
            red: i32;
            green: i32;
            blue: i32;
            if (this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
            {
              red = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Red;
              green = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Green;
              blue = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Blue;
            }
            if (red != 0 | green != 0 | blue != 0 | isMilitia)
            {
              if (!flag2)
              {
                float num4 =  ((int) byte.MaxValue + red) / 256f;
                float num5 =  ((int) byte.MaxValue + green) / 256f;
                float num6 =  ((int) byte.MaxValue + blue) / 256f;
                if ( num4 > 1.0)
                  num4 = 1f;
                if ( num5 > 1.0)
                  num5 = 1f;
                if ( num6 > 1.0)
                  num6 = 1f;
                if (0.0 >  num4)
                  num1 = 0.0f;
                if (0.0 >  num5)
                  num2 = 0.0f;
                if (0.0 >  num6)
                  num3 = 0.0f;
                if (isMilitia)
                {
                  ref Graphics local3 = ref this.g2;
                  ref local4: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBigHigh;
                  rectangle1 = Rectangle::new(76 * landscapeType, 0, 76, 76);
                  let mut srcrect: &Rectangle = &rectangle1
                  rectangle2 = Rectangle::new(tx, ty, 76, 76);
                  let mut destrect: &Rectangle = &rectangle2
                  DrawMod.DrawSimplePart2(ref local3, ref local4, srcrect, destrect);
                }
                else
                {
                  ref Graphics local5 = ref this.g2;
                  ref local6: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBig;
                  rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, 76, 76);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local5, ref local6, srcrect, destrect);
                }
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx, ty, 74, 74, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 72, 72, 0, 0, 0, (int) byte.MaxValue);
              }
              else
              {
                float num7 =  ((int) byte.MaxValue + red) / 256f;
                float num8 =  ((int) byte.MaxValue + green) / 256f;
                float num9 =  ((int) byte.MaxValue + blue) / 256f;
                if ( num7 > 1.0)
                  num7 = 1f;
                if ( num8 > 1.0)
                  num8 = 1f;
                if ( num9 > 1.0)
                  num9 = 1f;
                if (0.0 >  num7)
                  num1 = 0.0f;
                if (0.0 >  num8)
                  num2 = 0.0f;
                if (0.0 >  num9)
                  num3 = 0.0f;
                if (isMilitia)
                {
                  ref Graphics local7 = ref this.g2;
                  ref local8: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBigHigh;
                  rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, 76, 76);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect, destrect);
                }
                else
                {
                  ref Graphics local9 = ref this.g2;
                  ref local10: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBig;
                  rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, 76, 76);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
                }
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx, ty, 74, 74, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 72, 72, 0, 0, 0, (int) byte.MaxValue);
              }
            }
            else if (!flag2)
            {
              ref Graphics local11 = ref this.g2;
              ref local12: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBig;
              rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, 76, 76);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect, destrect);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx, ty, 74, 74, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 72, 72, 0, 0, 0, (int) byte.MaxValue);
            }
            else
            {
              ref Graphics local13 = ref this.g2;
              ref local14: Bitmap = ref this.game.Data.RegimeObj[regime1].TempCounterBig;
              rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, 76, 76);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect, destrect);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx, ty, 74, 74, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
              if (flag3 & ShowAttacker)
                DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 72, 72, 0, 0, 0, (int) byte.MaxValue);
            }
          }
        }
        else
          goto label_403;
      }
      else if (OverruleHis > -1 & OverruleRegime > -1)
      {
        if (OverruleRegime > -1 && Information.IsNothing( this.game.Data.RegimeObj[OverruleRegime].TempCounterBig))
          this.game.Data.RegimeObj[OverruleRegime].DoTempCounterBig();
        let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType;
        if (isMilitia)
        {
          ref Graphics local15 = ref this.g2;
          ref local16: Bitmap = ref this.game.Data.RegimeObj[regime2].TempCounterBigHigh;
          rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx, ty, 76, 76);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
        }
        else
        {
          ref Graphics local17 = ref this.g2;
          ref local18: Bitmap = ref this.game.Data.RegimeObj[regime2].TempCounterBig;
          rectangle2 = Rectangle::new(76 * landscapeType, 0, 76, 76);
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(tx, ty, 76, 76);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect, destrect);
        }
      }
      let mut num10: i32 =  0;
      if (isMilitia & this.game.Data.Turn == regime1 & OverruleHis == -1)
        DrawMod.DrawTextColouredConsoleCenter(ref this.g2, "M", this.game.MarcFont4, tx + 66, ty + 2, Color.FromArgb(228, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
      else if (!isMilitia & this.game.Data.Turn == regime1 & OverruleHis == -1)
      {
        counterString: String = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].CounterString;
        if (Operators.CompareString(Strings.Trim(counterString), Strings.Trim(Conversion.Val(counterString).ToString()), false) == 0)
        {
          let mut num11: i32 =  (int) Math.Round( this.g2.MeasureString(counterString, DrawMod.TGame.MarcFont4).Width);
          num10 = num11;
          DrawMod.DrawTextColouredConsole(ref this.g2, counterString, this.game.MarcFont4, tx + 72 - num11, ty + 2, Color.FromArgb(228, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
        }
      }
      if (coordinate.x >= 2)
      {
        if (index1 > -1 & !this.game.Data.UnitObj[nr].IsHQ)
        {
          if (this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
          {
            let mut red: i32 =  this.game.Data.UnitObj[index1].Red;
            let mut green: i32 =  this.game.Data.UnitObj[index1].Green;
            let mut blue: i32 =  this.game.Data.UnitObj[index1].Blue;
            DrawMod.DrawBlock(ref this.g2, tx, ty + 53, 75, 21, red, green, blue, 158);
          }
        }
        else if (this.game.Data.UnitObj[nr].IsHQ & this.game.Data.UnitObj[nr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Type < 8)
        {
          let mut red: i32 =  this.game.Data.UnitObj[nr].Red;
          let mut green: i32 =  this.game.Data.UnitObj[nr].Green;
          let mut blue: i32 =  this.game.Data.UnitObj[nr].Blue;
          DrawMod.DrawBlockGradient2(ref this.g2, tx, ty + 1, 75, 73, Color.FromArgb(0, red, green, blue), Color.FromArgb(155, red, green, blue));
        }
      }
      index3: i32;
      historical: i32;
      if (!mostlyHidden)
      {
        if (OverruleHis == -1)
        {
          let mut num12: i32 =  0;
          if (this.game.Data.UnitObj[nr].HQ == -1 & !this.game.Data.UnitObj[nr].IsHQ)
            num12 = 0;
          let mut num13: i32 =  -1;
          let mut num14: i32 =  -1;
          if (coordinate.x > 1)
          {
            if (!this.game.Data.UnitObj[nr].IsHQ)
            {
              let mut sfCount1: i32 =  this.game.Data.UnitObj[nr].SFCount;
              num15: i32;
              for (index3 = 0; index3 <= sfCount1; index3 += 1)
              {
                let mut sf: i32 =  this.game.Data.UnitObj[nr].SFList[index3];
                let mut index4: i32 =  this.game.Data.SFObj[sf].Type;
                let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                if (this.game.Data.UnitObj[nr].X > -1 && index4 > -1 && this.game.Data.SFTypeObj[index4].Theater != 1 & this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType].IsSea)
                  index4 = -1;
                if (index4 > -1)
                {
                  let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[index4].SymbolGroup;
                  let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[index4].SymbolWeight;
                  if (symbolGroup > -1)
                  {
                    numArray2[symbolGroup] = numArray2[symbolGroup] + symbolWeight * qty * 10;
                    if (this.game.Data.SFTypeObj[index4].CarryCap > 0)
                      numArray2[symbolGroup] = numArray2[symbolGroup] + 1;
                    if (numArray2[symbolGroup] > num14)
                    {
                      num14 = numArray2[symbolGroup];
                      num15 = symbolGroup;
                    }
                  }
                }
              }
              if (((num15 > -1 ? 1 : 0) | (this.game.Data.UnitObj[nr].Historical > -1 ? 1 : 0) & 0 | ( this.game.Data.RuleVar[344] == 0.0 ? 1 : 0)) != 0)
              {
                let mut num16: i32 =  num15;
                let mut sfTypeNr: i32 =  -1;
                let mut num17: i32 =  0;
                let mut index5: i32 =  -1;
                let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
                for (let mut index6: i32 =  0; index6 <= sfTypeCounter; index6 += 1)
                  numArray2[index6] = 0;
                let mut sfCount2: i32 =  this.game.Data.UnitObj[nr].SFCount;
                for (let mut index7: i32 =  0; index7 <= sfCount2; index7 += 1)
                {
                  let mut sf: i32 =  this.game.Data.UnitObj[nr].SFList[index7];
                  let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                  let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                  let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[type].SymbolGroup;
                  let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[type].SymbolWeight;
                  if (symbolGroup == num16)
                  {
                    numArray2[type] = numArray2[type] + symbolWeight * qty * 10;
                    if (this.game.Data.SFTypeObj[type].CarryCap > 0)
                      numArray2[type] = numArray2[type] + 1;
                    if (numArray2[type] > num17)
                    {
                      num17 = numArray2[type];
                      sfTypeNr = type;
                      index5 = sf;
                    }
                  }
                }
                if (sfTypeNr > -1)
                {
                  if (this.game.EditObj.HideUnit != 2 & sfTypeNr > -1 & this.game.Data.UnitObj[nr].Historical > -1 & this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[89] > 0)
                  {
                    let mut tv0: i32 =  this.game.Data.PeopleObj[this.game.Data.SFObj[index5].People].tv0;
                    if (this.slotCulture < 0)
                      this.slotCulture = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
                    let mut integer2: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.slotCulture].GetData(0, tv0, 1));
                    objBitmap: Bitmap;
                    if (!Information.IsNothing( this.game.Data.UnitObj[nr].tempSFTypeBitmap))
                    {
                      objBitmap = this.game.Data.UnitObj[nr].tempSFTypeBitmap;
                    }
                    else
                    {
                      objBitmap = this.DrawSFTypeGraphic(sfTypeNr, isMilitia, integer2, regime1, nr);
                      this.game.Data.UnitObj[nr].tempSFTypeBitmap = objBitmap;
                    }
                    if (!Information.IsNothing( objBitmap))
                    {
                      let mut num18: i32 =  0;
                      let mut num19: i32 =  2;
                      let mut w: i32 =  76;
                      let mut h: i32 =  64;
                      let mut width: i32 =  objBitmap.Width;
                      let mut height: i32 =  objBitmap.Height;
                      if (width > w | height > h)
                      {
                        if ( width /  w >  height /  h)
                        {
                          float num20 =  w /  width;
                          let mut num21: i32 =  (int) Math.Round( ( h -  height * num20));
                          num19 += (int) Math.Round( num21 / 2.0);
                          h -= num21;
                        }
                        else
                        {
                          float num22 =  h /  height;
                          let mut num23: i32 =  (int) Math.Round( ( w -  width * num22));
                          num18 += (int) Math.Round( num23 / 2.0);
                          w -= num23;
                        }
                        if (this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Mirror)
                        {
                          Matrix matrix = Matrix::new();
                          matrix.Scale(-1f, 1f);
                          if (Information.IsNothing( toG))
                            matrix.Translate( -(w + 0), 0.0f);
                          else
                            matrix.Translate( -(2 * (tx + num18) + (w + 0)), 0.0f);
                          this.g2.Transform = matrix;
                        }
                        DrawMod.DrawScaled(ref this.g2, ref objBitmap, tx + num18, ty + num19, w, h, true);
                      }
                      else
                      {
                        if (this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Mirror)
                        {
                          Matrix matrix = Matrix::new();
                          matrix.Scale(-1f, 1f);
                          if (Information.IsNothing( toG))
                            matrix.Translate( -(w + 0), 0.0f);
                          else
                            matrix.Translate( -(2 * tx + (w + 0)), 0.0f);
                          this.g2.Transform = matrix;
                        }
                        DrawMod.DrawSimple(ref this.g2, ref objBitmap, tx + num18 + (int) Math.Round( (w - width) / 2.0), ty + num19 + (int) Math.Round( (h - height) / 2.0));
                      }
                      this.g2.ResetTransform();
                    }
                  }
                  else if (((sfTypeNr > -1 ? 1 : 0) | (this.game.Data.UnitObj[nr].Historical > -1 ? 1 : 0) & 0) != 0)
                  {
                    let mut nr1: i32 =  -1;
                    let mut nr2: i32 =  -1;
                    if ( this.game.Data.RuleVar[871] > 0.0 & sfTypeNr > -1)
                    {
                      if (this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Mirror)
                      {
                        Matrix matrix = Matrix::new();
                        matrix.Scale(-1f, 1f);
                        if (Information.IsNothing( toG))
                          matrix.Translate(-90f, 0.0f);
                        else
                          matrix.Translate( -(2 * tx + 90), 0.0f);
                        this.g2.Transform = matrix;
                      }
                      nr1 = -1;
                      if (sfTypeNr > -1)
                        nr1 = this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigSpriteID;
                      if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigFileName, "systemgraphics/trans.bmp", false) == 0)
                        nr1 = -1;
                      if (regime1 > -1 & sfTypeNr > -1)
                      {
                        if (this.game.Data.RegimeObj[regime1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index8: i32 =  0; index8 <= extraCounter; index8 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index8] == this.game.Data.RegimeObj[regime1].ExtraGraphicUse)
                            {
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigSpriteID[index8];
                              if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigFileName[index8], "systemgraphics/trans.bmp", false) == 0)
                                nr1 = -1;
                            }
                          }
                        }
                        else if (this.game.Data.PeopleObj[integer1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index9: i32 =  0; index9 <= extraCounter; index9 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index9] == this.game.Data.PeopleObj[integer1].ExtraGraphicUse)
                            {
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigSpriteID[index9];
                              if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigFileName[index9], "systemgraphics/trans.bmp", false) == 0)
                                nr1 = -1;
                            }
                          }
                        }
                      }
                      if (nr1 > -1)
                      {
                        let mut num24: i32 =  0;
                        let mut num25: i32 =  0;
                        let mut width: i32 =  BitmapStore.GetWidth(nr1);
                        let mut num26: i32 =  BitmapStore.Getheight(nr1);
                        if (width < 70)
                        {
                          num24 = 74 - width;
                          num25 = 4;
                        }
                        let mut baseColor: i32 =  this.game.Data.SFTypeObj[sfTypeNr].BaseColor;
                        float red =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Red;
                        float green =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Green;
                        float blue =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Blue;
                        switch (baseColor)
                        {
                          case 0:
                            ref Graphics local19 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local20: Bitmap = ref bitmap1;
                            let mut x1: i32 =  tx + num24;
                            let mut y1: i32 =  ty + num25;
                            let mut w1: i32 =  width;
                            let mut h1: i32 =  num26;
                            DrawMod.DrawScaled(ref local19, ref local20, x1, y1, w1, h1);
                            break;
                          case 1:
                            ref Graphics local21 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local22: Bitmap = ref bitmap1;
                            let mut x2: i32 =  tx + num24;
                            let mut y2: i32 =  ty + num25;
                            let mut w2: i32 =  width;
                            let mut h2: i32 =  num26;
                            let mut origw1: i32 =  width;
                            let mut origh1: i32 =  num26;
                            double r1 =  red / 256.0;
                            double g1 =  green / 256.0;
                            double b1 =  blue / 256.0;
                            DrawMod.DrawScaledColorized2(ref local21, ref local22, x2, y2, w2, h2, origw1, origh1,  r1,  g1,  b1, 1f);
                            break;
                          case 2:
                            float red2 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Red2;
                            float green2 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Green2;
                            float blue2 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Blue2;
                            ref Graphics local23 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local24: Bitmap = ref bitmap1;
                            let mut x3: i32 =  tx + num24;
                            let mut y3: i32 =  ty + num25;
                            let mut w3: i32 =  width;
                            let mut h3: i32 =  num26;
                            let mut origw2: i32 =  width;
                            let mut origh2: i32 =  num26;
                            double r2 =  red2 / 256.0;
                            double g2 =  green2 / 256.0;
                            double b2 =  blue2 / 256.0;
                            DrawMod.DrawScaledColorized2(ref local23, ref local24, x3, y3, w3, h3, origw2, origh2,  r2,  g2,  b2, 1f);
                            break;
                          case 3:
                            float red3 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Red3;
                            float green3 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Green3;
                            float blue3 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Blue3;
                            ref Graphics local25 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local26: Bitmap = ref bitmap1;
                            let mut x4: i32 =  tx + num24;
                            let mut y4: i32 =  ty + num25;
                            let mut w4: i32 =  width;
                            let mut h4: i32 =  num26;
                            let mut origw3: i32 =  width;
                            let mut origh3: i32 =  num26;
                            double r3 =  red3 / 256.0;
                            double g3 =  green3 / 256.0;
                            double b3 =  blue3 / 256.0;
                            DrawMod.DrawScaledColorized2(ref local25, ref local26, x4, y4, w4, h4, origw3, origh3,  r3,  g3,  b3, 1f);
                            break;
                          case 4:
                            float red4 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Red4;
                            float green4 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Green4;
                            float blue4 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Blue4;
                            ref Graphics local27 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local28: Bitmap = ref bitmap1;
                            let mut x5: i32 =  tx + num24;
                            let mut y5: i32 =  ty + num25;
                            let mut w5: i32 =  width;
                            let mut h5: i32 =  num26;
                            let mut origw4: i32 =  width;
                            let mut origh4: i32 =  num26;
                            double r4 =  red4 / 256.0;
                            double g4 =  green4 / 256.0;
                            double b4 =  blue4 / 256.0;
                            DrawMod.DrawScaledColorized2(ref local27, ref local28, x5, y5, w5, h5, origw4, origh4,  r4,  g4,  b4, 1f);
                            break;
                          case 5:
                            ref Graphics local29 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local30: Bitmap = ref bitmap1;
                            let mut x6: i32 =  tx + num24;
                            let mut y6: i32 =  ty + num25;
                            let mut w6: i32 =  width;
                            let mut h6: i32 =  num26;
                            let mut origw5: i32 =  width;
                            let mut origh5: i32 =  num26;
                            double r5 = ( red + 392.0) / 1024.0;
                            double g5 = ( green + 392.0) / 1024.0;
                            double b5 = ( blue + 392.0) / 1024.0;
                            DrawMod.DrawScaledColorized2(ref local29, ref local30, x6, y6, w6, h6, origw5, origh5,  r5,  g5,  b5, 1f);
                            break;
                          case 6:
                            ref Graphics local31 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1);
                            ref local32: Bitmap = ref bitmap1;
                            let mut x7: i32 =  tx + num24;
                            let mut y7: i32 =  ty + num25;
                            let mut w7: i32 =  width;
                            let mut h7: i32 =  num26;
                            let mut origw6: i32 =  width;
                            let mut origh6: i32 =  num26;
                            double r6 = ( red + 80.0) / 512.0;
                            double g6 = ( green + 200.0) / 512.0;
                            double b6 = ( blue + 80.0) / 512.0;
                            DrawMod.DrawScaledColorized2(ref local31, ref local32, x7, y7, w7, h7, origw6, origh6,  r6,  g6,  b6, 1f);
                            break;
                        }
                      }
                      nr2 = -1;
                      if (sfTypeNr > -1)
                        nr2 = this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigSprite2ID;
                      if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigFileName2, "systemgraphics/trans.bmp", false) == 0)
                        nr2 = -1;
                      if (regime1 > -1 & sfTypeNr > -1)
                      {
                        if (this.game.Data.RegimeObj[regime1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index10: i32 =  0; index10 <= extraCounter; index10 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index10] == this.game.Data.RegimeObj[regime1].ExtraGraphicUse)
                            {
                              nr2 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigSprite2ID[index10];
                              if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigFileName2[index10], "systemgraphics/trans.bmp", false) == 0)
                                nr2 = -1;
                            }
                          }
                        }
                        else if (this.game.Data.PeopleObj[integer1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index11: i32 =  0; index11 <= extraCounter; index11 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index11] == this.game.Data.PeopleObj[integer1].ExtraGraphicUse)
                            {
                              nr2 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigSprite2ID[index11];
                              if (Operators.CompareString(this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolColBigFileName2[index11], "systemgraphics/trans.bmp", false) == 0)
                                nr2 = -1;
                            }
                          }
                        }
                      }
                      if (nr2 > -1)
                      {
                        let mut num27: i32 =  0;
                        let mut num28: i32 =  0;
                        if (BitmapStore.GetWidth(nr2) < 70)
                        {
                          num27 = 74 - BitmapStore.GetWidth(nr2);
                          num28 = 4;
                        }
                        ref Graphics local33 = ref this.g2;
                        bitmap1 = BitmapStore.GetBitmap(nr2);
                        ref local34: Bitmap = ref bitmap1;
                        let mut x: i32 =  tx + num27;
                        let mut y: i32 =  ty + num28;
                        DrawMod.DrawSimple(ref local33, ref local34, x, y);
                      }
                      this.g2.ResetTransform();
                    }
                    if ( this.game.Data.RuleVar[871] <= 0.0 | nr2 == -1 & nr1 == -1)
                    {
                      if (sfTypeNr > -1)
                        nr1 = this.game.Data.SFTypeObj[sfTypeNr].SymbolSpriteID;
                      let mut nr3: i32 =  -1;
                      if (regime1 > -1 & sfTypeNr > -1)
                      {
                        if (this.game.Data.RegimeObj[regime1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index12: i32 =  0; index12 <= extraCounter; index12 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index12] == this.game.Data.RegimeObj[regime1].ExtraGraphicUse)
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index12];
                          }
                        }
                        else if (this.game.Data.PeopleObj[integer1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index13: i32 =  0; index13 <= extraCounter; index13 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index13] == this.game.Data.PeopleObj[integer1].ExtraGraphicUse)
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index13];
                          }
                        }
                      }
                      num29: i32;
                      if (this.game.EditObj.HideUnit == 2 && this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
                      {
                        nr1 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx <= -1 ? -1 : this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx];
                        if (nr1 == -1 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter > 0 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= this.game.NATO.GetUpperBound(0))
                          nr1 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter];
                        num29 = 12;
                        if (this.game.Data.UnitObj[nr].HistoricalSubPart > -1)
                        {
                          if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                            nr3 = this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                          else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                            nr3 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                        }
                      }
                      num13 = nr1;
                      let mut regime3: i32 =  this.game.Data.UnitObj[nr].Regime;
                      if (this.game.EditObj.PrefMinimalistCounter && this.game.Data.Product < 6)
                        num12 -= 7;
                      if (((this.game.EditObj.PrefMinimalistCounter &  this.game.Data.RuleVar[999] == 1.0 ? 1 : 0) & 1 & (!this.game.Data.UnitObj[nr].IsHQ ? 1 : 0)) != 0 && this.game.Data.Product < 6)
                        num29 += 6;
                      if ((( this.game.Data.RuleVar[847] == 1.0 ? 1 : 0) & 0) != 0)
                        num29 -= 6;
                      if ((( this.game.Data.RuleVar[847] == 1.0 ? 1 : 0) & 1) != 0)
                        num29 += 5;
                      let mut num30: i32 =  num29 + 2;
                      if (regime3 > -1 & sfTypeNr > -1)
                      {
                        float num31;
                        float num32;
                        float num33;
                        if (this.game.Data.SFTypeObj[sfTypeNr].SymbolOverrule)
                        {
                          num31 = 1f;
                          num32 = 1f;
                          num33 = 1f;
                        }
                        else
                        {
                          num31 = 1f;
                          num32 = 1f;
                          num33 = 1f;
                        }
                        if (this.game.EditObj.HideUnit != 2 & this.game.Data.RegimeObj[regime3].Mirror & (0 & (this.game.Data.UnitObj[nr].Historical > -1 ? 1 : 0)) == 0)
                        {
                          Matrix matrix = Matrix::new();
                          matrix.Scale(-1f, 1f);
                          if (Information.IsNothing( toG))
                            matrix.Translate(-90f, 0.0f);
                          else
                            matrix.Translate( -(2 * tx + 90), 0.0f);
                          this.g2.Transform = matrix;
                          if (this.game.EditObj.PrefMinimalistCounter && this.game.Data.Product < 6)
                            num12 = 9;
                          if (this.game.Data.Product == 7)
                            num12 += 10;
                        }
                        if (this.game.EditObj.HideUnit == 2)
                          num12 -= 7;
                        if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[82] > 0 & this.game.EditObj.HideUnit != 2)
                        {
                          num30 = -2;
                          num12 += 3;
                        }
                        if ( num31 == 1.0 &  num32 == 1.0 &  num33 == 1.0)
                        {
                          if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[82] > 0 & nr1 > 0)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].artCode[0] < 1 | this.game.EditObj.HideUnit == 2)
                            {
                              ref Graphics local35 = ref this.g2;
                              bitmap1 = BitmapStore.GetBitmap(nr1, 1);
                              ref local36: Bitmap = ref bitmap1;
                              let mut x: i32 =  num12 + tx;
                              let mut y: i32 =  ty + num30;
                              DrawMod.DrawSimple(ref local35, ref local36, x, y);
                            }
                            else
                            {
                              ref Graphics local37 = ref this.g2;
                              bitmap1 = BitmapStore.GetBitmap(nr1, 1);
                              ref local38: Bitmap = ref bitmap1;
                              rectangle2 = Rectangle::new(0, 0, 76, 76);
                              let mut srcrect: &Rectangle = &rectangle2
                              rectangle1 = Rectangle::new(num12 + tx, ty + num30, 76, 76);
                              let mut destrect: &Rectangle = &rectangle1
                              double r =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[1] /  byte.MaxValue);
                              double g =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[2] /  byte.MaxValue);
                              double b =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[3] /  byte.MaxValue);
                              DrawMod.DrawSimplePart2ColouredNew(ref local37, ref local38, srcrect, destrect,  r,  g,  b, 1f);
                            }
                            if (this.game.Data.SFTypeObj[sfTypeNr].artCode[5] == 1 & this.game.EditObj.HideUnit != 2)
                            {
                              ref Graphics local39 = ref this.g2;
                              bitmap1 = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigSprite2ID);
                              ref local40: Bitmap = ref bitmap1;
                              rectangle2 = Rectangle::new(0, 0, 76, 76);
                              let mut srcrect: &Rectangle = &rectangle2
                              rectangle1 = Rectangle::new(num12 + tx, ty + num30, 76, 76);
                              let mut destrect: &Rectangle = &rectangle1
                              double r =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[6] /  byte.MaxValue);
                              double g =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[7] /  byte.MaxValue);
                              double b =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[8] /  byte.MaxValue);
                              double a =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[9] /  byte.MaxValue);
                              DrawMod.DrawSimplePart2ColouredNew(ref local39, ref local40, srcrect, destrect,  r,  g,  b,  a);
                            }
                          }
                          else if (nr1 > 0)
                          {
                            ref Graphics local41 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1, 1);
                            ref local42: Bitmap = ref bitmap1;
                            let mut x: i32 =  num12 + tx;
                            let mut y: i32 =  ty + num30;
                            DrawMod.DrawSimple(ref local41, ref local42, x, y);
                          }
                          this.g2.ResetTransform();
                          if (nr3 > 0)
                          {
                            ref Graphics local43 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr3, 1);
                            ref local44: Bitmap = ref bitmap1;
                            let mut x: i32 =  num12 + tx;
                            let mut y: i32 =  ty + num30;
                            DrawMod.DrawSimple(ref local43, ref local44, x, y);
                          }
                        }
                        else
                        {
                          if (nr1 > 0)
                          {
                            ref Graphics local45 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr1, 1);
                            ref local46: Bitmap = ref bitmap1;
                            let mut x: i32 =  num12 + tx;
                            let mut y: i32 =  ty + num30;
                            double r =  num31 - 1.0;
                            double g =  num32 - 1.0;
                            double b =  num33 - 1.0;
                            DrawMod.Draw(ref local45, ref local46, x, y,  r,  g,  b, 1f);
                          }
                          this.g2.ResetTransform();
                          if (nr3 > 0)
                          {
                            ref Graphics local47 = ref this.g2;
                            bitmap1 = BitmapStore.GetBitmap(nr3, 1);
                            ref local48: Bitmap = ref bitmap1;
                            let mut x: i32 =  num12 + tx;
                            let mut y: i32 =  ty + num30;
                            double r =  num31 - 1.0;
                            double g =  num32 - 1.0;
                            double b =  num33 - 1.0;
                            DrawMod.Draw(ref local47, ref local48, x, y,  r,  g,  b, 1f);
                          }
                        }
                      }
                      else
                      {
                        if (nr1 > 0)
                        {
                          ref Graphics local49 = ref this.g2;
                          bitmap1 = BitmapStore.GetBitmap(nr1, 1);
                          ref local50: Bitmap = ref bitmap1;
                          let mut x: i32 =  num12 + tx;
                          let mut y: i32 =  ty + num30;
                          DrawMod.DrawSimple(ref local49, ref local50, x, y);
                        }
                        this.g2.ResetTransform();
                        if (nr3 > 0)
                        {
                          ref Graphics local51 = ref this.g2;
                          bitmap1 = BitmapStore.GetBitmap(nr3, 1);
                          ref local52: Bitmap = ref bitmap1;
                          let mut x: i32 =  num12 + tx;
                          let mut y: i32 =  ty + num30;
                          DrawMod.DrawSimple(ref local51, ref local52, x, y);
                        }
                      }
                      if (this.game.EditObj.HideUnit == 2)
                      {
                        historical = this.game.Data.UnitObj[nr].Historical;
                        counterString: String = this.game.Data.HistoricalUnitObj[historical].CounterString;
                        str1: String = Strings.UCase(Strings.Left(counterString, 1)) + Strings.Mid(counterString, 2);
                        str2: String = Conversion.Str( Conversion.Val(str1));
                        str3: String;
                        if (Operators.CompareString(Strings.Trim(str1), Strings.Trim(str2), false) == 0)
                        {
                          let mut num34: i32 =  (int) Math.Round(Conversion.Val(str1));
                          str3 = !((num34 + 10) % 10 == 1 & (num34 + 100) % 100 != 11) ? (!((num34 + 10) % 10 == 2 & (num34 + 100) % 100 != 12) ? (!((num34 + 10) % 10 == 3 & (num34 + 100) % 100 != 13) ? str2 + "th" : str2 + "rd") : str2 + "nd") : str2 + "st";
                        }
                        else
                          str3 = str1;
                        let mut num35: i32 =  (int) Math.Round(38.0 -  (int) Math.Round( this.g2.MeasureString(str3, DrawMod.TGame.MarcFont16).Width) / 2.0);
                        DrawMod.DrawTextColouredMarc(ref this.g2, Strings.Trim(str3), DrawMod.TGame.MarcFont16, tx + num35, ty + 8, Color.White);
                      }
                    }
                  }
                }
              }
            }
            else if (this.game.Data.UnitObj[nr].Regime > -1)
            {
              let mut hqSpriteNr: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr;
              float num36 =  this.game.Data.RegimeObj[index2].Red / 256f;
              float num37 =  this.game.Data.RegimeObj[index2].Green / 256f;
              float num38 =  this.game.Data.RegimeObj[index2].Blue / 256f;
              ref Graphics local53 = ref this.g2;
              bitmap1 = BitmapStore.GetBitmap(hqSpriteNr, 1);
              ref local54: Bitmap = ref bitmap1;
              let mut x8: i32 =  tx - 8;
              let mut y8: i32 =  ty + 4;
              double r7 =  num36 - 1.0;
              double g7 =  num37 - 1.0;
              double b7 =  num38 - 1.0;
              DrawMod.Draw(ref local53, ref local54, x8, y8,  r7,  g7,  b7, 1f);
              let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr2;
              if (this.game.Data.UnitObj[nr].Historical <= -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx > -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= 0)
                ;
              float num39;
              float num40;
              float num41;
              if (!this.game.Data.RegimeObj[index2].HQSpriteOverrule)
              {
                num39 =  this.game.Data.RegimeObj[index2].Red3 / 256f;
                num40 =  this.game.Data.RegimeObj[index2].Green3 / 256f;
                num41 =  this.game.Data.RegimeObj[index2].Blue3 / 256f;
              }
              else
              {
                num39 = 1f;
                num40 = 1f;
                num41 = 1f;
              }
              ref Graphics local55 = ref this.g2;
              bitmap1 = BitmapStore.GetBitmap(hqSpriteNr2, 1);
              ref local56: Bitmap = ref bitmap1;
              let mut x9: i32 =  tx - 8;
              let mut y9: i32 =  ty + 4;
              double r8 =  num39 - 1.0;
              double g8 =  num40 - 1.0;
              double b8 =  num41 - 1.0;
              DrawMod.Draw(ref local55, ref local56, x9, y9,  r8,  g8,  b8, 1f);
              historical = this.game.Data.UnitObj[nr].Historical;
              counterString: String = this.game.Data.HistoricalUnitObj[historical].CounterString;
              str: String = Strings.UCase(Strings.Left(counterString, 1)) + Strings.Mid(counterString, 2);
              Conversion.Str( Conversion.Val(str));
              let mut num42: i32 =  (int) Math.Round(38.0 -  (int) Math.Round( this.g2.MeasureString(str, DrawMod.TGame.MarcFont16).Width) / 2.0);
              DrawMod.DrawTextColouredMarc(ref this.g2, Strings.Trim(str), DrawMod.TGame.MarcFont16, tx + num42, ty + 47, Color.White);
            }
          }
        }
        else
        {
          let mut num43: i32 =  OverruleHis > -1 & OverrulePower != -9999 &  this.game.Data.RuleVar[344] == 1.0 ? 1 : 0;
        }
      }
      if (OverruleHis == -1 & !mostlyHidden)
      {
        regime2 = this.game.Data.UnitObj[nr].Regime;
        float red2 =  this.game.Data.RegimeObj[index2].Red2;
        float green2 =  this.game.Data.RegimeObj[index2].Green2;
        float blue2 =  this.game.Data.RegimeObj[index2].Blue2;
        float num44 = 0.0f;
        float num45 = 0.0f;
        float num46 = 0.0f;
        if ( red2 < 128.0 &  green2 < 128.0 &  blue2 < 128.0)
        {
          num44 =  byte.MaxValue;
          num45 =  byte.MaxValue;
          num46 =  byte.MaxValue;
        }
        index3 = coordinate.x != 2 ? ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon, AbsPower: true)) : ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y, AbsPower: true));
      }
      if (index3 > 999)
        index3 = 999;
      if (OverrulePower > -1)
        index3 = OverrulePower;
      else if (OverrulePower == -9999)
        index3 = -9999;
      let mut Number: i32 =  index3;
      if (Number > 99)
        Number = 99;
      if (OverruleHis > -1)
      {
        num1 =  this.game.Data.RegimeObj[OverruleRegime].Red2;
        num2 =  this.game.Data.RegimeObj[OverruleRegime].Green2;
        num3 =  this.game.Data.RegimeObj[OverruleRegime].Blue2;
      }
      if ((coordinate.x > 1 | OverruleHis > -1) & !mostlyHidden)
      {
        historical = this.game.Data.UnitObj[nr].Historical;
        if (!this.game.Data.UnitObj[nr].IsHQ && (( this.game.Data.RuleVar[999] == 1.0 ? 1 : 0) & 1 & (!this.game.Data.UnitObj[nr].IsHQ ? 1 : 0)) != 0 && !this.game.EditObj.PrefMinimalistCounter)
        {
          let mut num47: i32 =  4;
          if (this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
          {
            let mut averageRdn: i32 =  this.game.HandyFunctionsObj.GetAverageRdn(nr);
            let mut num48: i32 =  Number;
            this.game.HandyFunctionsObj.GetBreakPercent(nr);
            let mut num49: i32 =  (int) Math.Round( this.game.Data.RuleVar[307]);
            let mut num50: i32 =  this.game.HandyFunctionsObj.GetStartPower(nr);
            if (num50 == 0)
              num50 = num48;
            let mut num51: i32 =  (int) Math.Round(Conversion.Int( num48 /  num50 * 100.0));
            if ((this.game.EditObj.HideUnit != 2 ? (int) Math.Round(34.0 - Conversion.Int( num48 /  num50 * 34.0)) : (int) Math.Round(62.0 - Conversion.Int( num48 /  num50 * 62.0))) < 0)
              ;
            if (num48 > 0)
            {
              float a1;
              float a2;
              float a3;
              if (averageRdn >= 75)
              {
                a1 = 0.0f;
                a2 =  byte.MaxValue;
                a3 = 0.0f;
              }
              else if (averageRdn >= 50)
              {
                a1 =  byte.MaxValue;
                a2 =  byte.MaxValue;
                a3 = 0.0f;
              }
              else if (averageRdn >= 25)
              {
                a1 = 0.0f;
                a2 = 170f;
                a3 =  byte.MaxValue;
              }
              else
              {
                a1 =  byte.MaxValue;
                a2 = 0.0f;
                a3 = 0.0f;
              }
              let mut num52: i32 =  num51;
              if (num52 > 100)
                num52 = 100;
              if (this.game.EditObj.HideUnit == 2)
              {
                DrawMod.DrawBlock(ref this.g2, tx + 6 + num47, ty + 60, 63, 11, 0, 0, 0, 98);
                DrawMod.DrawBlock(ref this.g2, tx + 6 + num47, ty + 61, (int) Math.Round(62.0 * ( num52 / 100.0)), 9, (int) Math.Round( a1), (int) Math.Round( a2), (int) Math.Round( a3), 150);
              }
              else
              {
                DrawMod.DrawBlock(ref this.g2, tx + 31 + num47, ty + 60, 35, 11, 0, 0, 0, 98);
                DrawMod.DrawBlock(ref this.g2, tx + 31 + num47, ty + 61, (int) Math.Round(34.0 * ( num52 / 100.0)), 9, (int) Math.Round( a1), (int) Math.Round( a2), (int) Math.Round( a3), 150);
              }
            }
          }
          if (this.game.EditObj.HideUnit != 2)
          {
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx > -1)
            {
              let mut nr4: i32 =  this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx];
              ref Graphics local57 = ref this.g2;
              bitmap1 = BitmapStore.GetBitmap(nr4);
              ref local58: Bitmap = ref bitmap1;
              let mut x: i32 =  tx - 2 + num47;
              let mut y: i32 =  ty + 41;
              DrawMod.DrawSimple(ref local57, ref local58, x, y);
            }
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart] > -1)
            {
              let mut nr5: i32 =  this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart]];
              if (nr5 > -1)
              {
                ref Graphics local59 = ref this.g2;
                bitmap1 = BitmapStore.GetBitmap(nr5);
                ref local60: Bitmap = ref bitmap1;
                let mut x: i32 =  tx - 2 + num47;
                let mut y: i32 =  ty + 41;
                DrawMod.DrawSimple(ref local59, ref local60, x, y);
              }
            }
          }
          str: String = Strings.Trim(Conversion.Str( Number));
          let mut num53: i32 =  (int) Math.Round(38.0 -  this.g2.MeasureString(str, DrawMod.TGame.MarcFont16).Width / 2.0);
          DrawMod.DrawTextColouredMarc(ref this.g2, str, DrawMod.TGame.MarcFont16, tx + num53 + num47, ty + 56, Color.White);
        }
      }
      if (OverruleHis == -1 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
      {
        float maxValue1 =  byte.MaxValue;
        float maxValue2 =  byte.MaxValue;
        float maxValue3 =  byte.MaxValue;
        let mut a4: i32 =  196;
        let mut a5: i32 =  92;
        float a6 = maxValue1 +  (int) Math.Round(( byte.MaxValue -  maxValue1) / 2.0);
        float a7 = maxValue2 +  (int) Math.Round(( byte.MaxValue -  maxValue2) / 2.0);
        float a8 = maxValue3 +  (int) Math.Round(( byte.MaxValue -  maxValue3) / 2.0);
        let mut num54: i32 =  (int) Math.Round(Math.Floor( this.game.HandyFunctionsObj.GetLowestAp(nr) / 10.0));
        if (num54 > 10)
          num54 = 10;
        if (isMilitia & num54 > 9)
          num54 = 9;
        if ((int) Math.Round(10.0 -  num10 / 6.0) < num54)
          num54 = (int) Math.Round(10.0 -  num10 / 6.0);
        if (!this.game.Data.UnitObj[nr].DidAttack & !this.game.Data.UnitObj[nr].DidMove)
        {
          a4 = (int) byte.MaxValue;
          a5 = (int) byte.MaxValue;
          a7 =  byte.MaxValue;
          a6 = 0.0f;
          a8 = 0.0f;
        }
        let mut num55: i32 =  num54;
        for (let mut index14: i32 =  1; index14 <= num55; index14 += 1)
        {
          DrawMod.DrawBlock(ref this.g2, tx + 6 + (index14 - 1) * 6, ty + 5, 3, 3, (int) Math.Round( a6), (int) Math.Round( a7), (int) Math.Round( a8), a4);
          DrawMod.DrawRectangle(ref this.g2, tx + 6 + (index14 - 1) * 6 - 1, ty + 5 - 1, 4, 4, 0, 0, 0, a5);
        }
        if (this.game.Data.UnitObj[nr].cycleOrder < 0L & this.game.Data.UnitObj[nr].Regime == this.game.EditObj.RealTurn)
        {
          DrawMod.DrawBlock(ref this.g2, tx + 5, ty + 10, 13, 14, 0, 0, 0, 200);
          DrawMod.DrawTextColouredMarcCenter(ref this.g2, "G", this.game.MarcFont4, tx + 12, ty + 8, Color.White);
        }
        if (this.game.EventRelatedObj.Helper_AirEnabled() & !this.game.EditObj.AIMoving && this.game.EditObj.Zoom >= 0 & OverruleHis == -1)
        {
          this.slotAir = this.strId534slot;
          let mut num56: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(55);
          let mut num57: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(56);
          let mut num58: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(57);
          let mut numy: i32 =  -1;
          let mut num59: i32 =  -1;
          if (this.slotAir > -1 & num56 > 0)
          {
            let mut airRowNr: i32 =  -1;
            let mut length: i32 =  this.game.Data.StringListObj[this.slotAir].Length;
            for (let mut index15: i32 =  0; index15 <= length; index15 += 1)
            {
              if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 0])) == this.game.Data.RegimeObj[this.game.Data.Turn].id && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 1])) == this.game.Data.UnitObj[nr].X && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 2])) == this.game.Data.UnitObj[nr].Y && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 3])) == num57 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 4])) == num58 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 9])) >= 0)
              {
                numy = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 8]));
                num59 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index15, 9]));
                airRowNr = index15;
                break;
              }
            }
            if (numy > -1)
            {
              letter: String = this.game.HandyFunctionsObj.CovertNumberToLetter(numy);
              color: Color = this.game.HandyFunctionsObj.Air_GetColor(airRowNr);
              let mut tcol: i32 =  0;
              let mut num60: i32 =  1;
              DrawMod.DrawBlock(ref this.g2, tx + num60, ty + 44, 18, 14, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
              DrawMod.DrawTextCenterSmallLabel(ref this.g2, letter, this.game.MarcFont4, tx - 1 + num60 + 10, ty + 52, tcol);
            }
          }
        }
        float a9;
        float a10;
        float a11;
        if (this.game.Data.UnitObj[nr].SupplyInReq > 0)
        {
          let mut num61: i32 =  (int) Math.Round(Conversion.Int( (this.game.Data.UnitObj[nr].SupplyIn * 100) /  this.game.Data.UnitObj[nr].SupplyInReq));
          if (num61 >= 100)
          {
            a9 = 0.0f;
            a10 =  byte.MaxValue;
            a11 = 0.0f;
          }
          else if (num61 >= 66)
          {
            a9 =  byte.MaxValue;
            a10 =  byte.MaxValue;
            a11 = 0.0f;
          }
          else if (num61 >= 44)
          {
            a9 = 0.0f;
            a10 = 170f;
            a11 =  byte.MaxValue;
          }
          else if (num61 >= 22)
          {
            a9 =  byte.MaxValue;
            a10 = 0.0f;
            a11 = 0.0f;
          }
          else
          {
            a9 = 0.0f;
            a10 = 0.0f;
            a11 = 0.0f;
          }
        }
        else if (this.game.Data.UnitObj[nr].HQ == -1 | this.game.Data.UnitObj[nr].SupplyInReq == 0)
        {
          a9 = 0.0f;
          a10 =  byte.MaxValue;
          a11 = 0.0f;
        }
        else
        {
          a9 = 0.0f;
          a10 = 0.0f;
          a11 = 0.0f;
        }
        if (this.game.Data.UnitObj[nr].IsHQ)
        {
          DrawMod.DrawBlock(ref this.g2, tx + 32, ty + 66, 12, 6, (int) Math.Round( a9), (int) Math.Round( a10), (int) Math.Round( a11), (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref this.g2, tx + 32, ty + 66, 12, 6, 0, 0, 0, 120);
        }
        else
        {
          DrawMod.DrawBlock(ref this.g2, tx + 4, ty + 63, 6, 6, (int) Math.Round( a9), (int) Math.Round( a10), (int) Math.Round( a11), (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref this.g2, tx + 4, ty + 63, 6, 6, 0, 0, 0, 120);
        }
      }
      if (OverruleHis == -1 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
      {
        if (this.game.EditObj.ShowSameHistorical && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[nr].HQ > -1 & this.game.Data.UnitObj[nr].HQ == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ & !this.game.Data.UnitObj[nr].IsHQ | nr == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
        {
          float red =  this.game.Data.UnitObj[index1].Red;
          float green =  this.game.Data.UnitObj[index1].Green;
          float blue =  this.game.Data.UnitObj[index1].Blue;
          DrawMod.DrawRectangle(ref this.g2, tx + 2, ty + 2, 70, 70, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 5);
        }
        if (this.game.EditObj.ShowUnderHQ && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.UnitObj[nr].HQ == this.game.EditObj.UnitSelected && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
        {
          float red =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Red;
          float green =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Green;
          float blue =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Blue;
          DrawMod.DrawRectangle(ref this.g2, tx + 2, ty + 2, 70, 70, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 5);
        }
        if ( this.game.Data.RuleVar[983] > 0.0 && !this.game.HandyFunctionsObj.CheckIfInCorrectFrontzone(nr, this.game.Data.UnitObj[nr].RealX(ref this.game), this.game.Data.UnitObj[nr].RealY(ref this.game)))
          DrawMod.DrawRectangle(ref this.g2, tx, ty, 77, 77, (int) byte.MaxValue, 0, 0, 172, 2);
      }
      if (this.game.EditObj.UnitSelected == nr & ShowAttacker)
      {
        DrawMod.DrawRectangle(ref this.g2, tx + 0, ty + 0, 74, 74, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 3);
        DrawMod.DrawRectangle(ref this.g2, tx - 1, ty - 1, 76, 76, 0, 0, 0, (int) byte.MaxValue);
        DrawMod.DrawRectangle(ref this.g2, tx + 2, ty + 2, 70, 70, 128, 128, 128, (int) byte.MaxValue);
      }
      if (Information.IsNothing( toG))
        return this.tmpbmp2;
label_403:
      bitmap2: Bitmap;
      return bitmap2;
    }

    pub DrawUnit: Bitmap(
      nr: i32,
      bool forcehighlight = false,
      Graphics toG = null,
      let mut tx: i32 =  0,
      let mut ty: i32 =  0,
      bool ShowAttacker = false,
      let mut OverruleHis: i32 =  -1,
      let mut OverrulePower: i32 =  -1,
      let mut OverruleRegime: i32 =  -1,
      bool FullRecon = false,
      let mut ForceHideUnitMode: i32 =  -1,
      bool mostlyHidden = false)
    {
      Coordinate coordinate = Coordinate::new();
      SizeF sizeF1 = SizeF::new();
      SizeF sizeF2 = SizeF::new();
      int[] numArray1 = new int[1];
      let mut hideUnit: i32 =  this.game.EditObj.HideUnit;
      if (ForceHideUnitMode > -1)
        this.game.EditObj.HideUnit = ForceHideUnitMode;
      int[] numArray2 = this.game.Data.SFTypeCounter >= 101 ? new int[this.game.Data.SFTypeCounter + 1] : new int[101];
      if ( this.game.Data.RuleVar[344] == 0.0 && this.game.EditObj.HideUnit == 2)
        this.game.EditObj.HideUnit = 1;
      if (!Information.IsNothing( toG))
      {
        this.g2 = toG;
      }
      else
      {
        this.g2 = Graphics.FromImage((Image) this.tmpbmp2);
        this.g2.Clear(Color.FromArgb(0, 0, 0, 0));
      }
      if (!this.game.Data.FOWOn)
        FullRecon = true;
      regime1: i32;
      if (nr == -1 & OverruleHis == -1)
      {
        let mut num: i32 =  1;
        DrawMod.DrawSimplePart2(ref this.g2, ref this.game.Data.RegimeObj[regime1].TempCounter, Rectangle::new(38 * num, 0, 38, 38), Rectangle::new(tx, ty, 38, 38));
      }
      else
      {
        bool isMilitia;
        if (OverruleHis == -1)
          isMilitia = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].GiveHisVarValue(11) > 0;
        else if (OverruleHis <= this.game.Data.HistoricalUnitCounter)
          isMilitia = this.game.Data.HistoricalUnitObj[OverruleHis].GiveHisVarValue(11) > 0;
        index1: i32;
        regimeNr: i32;
        integer1: i32;
        index2: i32;
        bitmap: Bitmap;
        float num1;
        float num2;
        float num3;
        Rectangle rectangle1;
        Rectangle rectangle2;
        num4: i32;
        if (OverruleHis == -1)
        {
          if (this.game.Data.UnitObj[nr].Historical != -1)
          {
            if (this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn | this.game.Data.Round < 1 | !this.game.Data.FOWOn)
              coordinate.x = 3;
            else
              coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(nr, this.game.Data.Turn);
            if (FullRecon)
            {
              coordinate.x = 3;
              coordinate.y = 0;
            }
            index1 = !this.game.Data.UnitObj[nr].IsHQ ? this.game.Data.UnitObj[nr].HQ : nr;
            bool flag1 = !(this.game.Data.UnitObj[nr].Regime == this.game.Data.Turn & this.game.Data.Round > 0) || this.game.Data.UnitObj[nr].X > -1 && this.game.HandyFunctionsObj.CanUnitMove2(nr);
            bool flag2 = false;
            if (Information.IsNothing( this.game.EditObj.TempUnitList))
              this.game.EditObj.TempUnitList = UnitList::new();
            bool flag3;
            if (this.game.EditObj.OrderType == 2 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 12 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 11 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 13 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 14 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 15 && this.game.EditObj.TempUnitList.CheckIfPresent(nr))
            {
              flag2 = false;
              flag3 = true;
            }
            if (this.game.EditObj.OrderType == 9)
            {
              if (this.game.EditObj.OrderUnit == nr)
                flag2 = true;
              if (this.game.EditObj.OrderTarget == nr)
                flag2 = true;
            }
            if (this.game.EditObj.OrderType == 3 && this.game.EditObj.OrderUnit == nr)
              flag2 = true;
            if (this.game.EditObj.OrderType == 19)
            {
              if (this.game.EditObj.OrderUnit == nr)
                flag2 = true;
              if (this.game.EditObj.OrderTarget == nr)
                flag2 = true;
            }
            if (this.game.EditObj.OrderType == 33 && this.game.EditObj.OrderUnit == nr)
              flag2 = true;
            if (forcehighlight)
              flag2 = true;
            if (this.game.EditObj.LayerSupplyOn && this.game.EditObj.LayerSupplyHQ == this.game.Data.UnitObj[nr].HQ)
              flag2 = true;
            if (this.game.Data.UnitObj[nr].TempUnitSelectable)
              flag2 = true;
            regimeNr = this.game.Data.UnitObj[nr].Regime;
            if (regimeNr == -1)
              regimeNr = 0;
            if (regimeNr == 0)
              regimeNr = regimeNr;
            integer1 = Conversions.ToInteger(this.game.HandyFunctionsObj.GetUnitPeople(nr));
            index2 = regimeNr;
            if (this.game.Data.PeopleObj[integer1].RegCol > -1)
              index2 = this.game.Data.PeopleObj[integer1].RegCol;
            if (Information.IsNothing( this.game.Data.RegimeObj[index2].TempCounter))
              this.game.Data.RegimeObj[index2].DoTempCounter();
            if (OverruleRegime > -1 && Information.IsNothing( this.game.Data.RegimeObj[OverruleRegime].TempCounter))
              this.game.Data.RegimeObj[OverruleRegime].DoTempCounter();
            if (this.game.Data.UnitObj[nr].Regime == -1)
            {
              ref Graphics local1 = ref this.g2;
              bitmap = BitmapStore.GetBitmap(this.game.DEFAULTCOUNTER);
              ref local2: Bitmap = ref bitmap;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local1, ref local2, x, y);
            }
            else
            {
              let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType;
              red: i32;
              green: i32;
              blue: i32;
              if (this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
              {
                red = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Red;
                green = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Green;
                blue = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Blue;
              }
              if (red != 0 | green != 0 | blue != 0 | isMilitia)
              {
                if (!flag2)
                {
                  float num5 =  ((int) byte.MaxValue + red) / 256f;
                  float num6 =  ((int) byte.MaxValue + green) / 256f;
                  float num7 =  ((int) byte.MaxValue + blue) / 256f;
                  if ( num5 > 1.0)
                    num5 = 1f;
                  if ( num6 > 1.0)
                    num6 = 1f;
                  if ( num7 > 1.0)
                    num7 = 1f;
                  if (0.0 >  num5)
                    num1 = 0.0f;
                  if (0.0 >  num6)
                    num2 = 0.0f;
                  if (0.0 >  num7)
                    num3 = 0.0f;
                  if (isMilitia)
                  {
                    ref Graphics local3 = ref this.g2;
                    ref local4: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounterHigh;
                    rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                    let mut srcrect: &Rectangle = &rectangle1
                    rectangle2 = Rectangle::new(tx, ty, 38, 38);
                    let mut destrect: &Rectangle = &rectangle2
                    DrawMod.DrawSimplePart2(ref local3, ref local4, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local5 = ref this.g2;
                    ref local6: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounter;
                    rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                    let mut srcrect: &Rectangle = &rectangle1
                    rectangle2 = Rectangle::new(tx, ty, 38, 38);
                    let mut destrect: &Rectangle = &rectangle2
                    DrawMod.DrawSimplePart2(ref local5, ref local6, srcrect, destrect);
                  }
                  if (flag3 & ShowAttacker)
                    DrawMod.DrawRectangle(ref this.g2, tx, ty, 38, 38, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                  if (flag3 & ShowAttacker)
                    DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 37, 37, 0, 0, 0, (int) byte.MaxValue);
                }
                else
                {
                  float num8 =  ((int) byte.MaxValue + red) / 256f;
                  float num9 =  ((int) byte.MaxValue + green) / 256f;
                  float num10 =  ((int) byte.MaxValue + blue) / 256f;
                  if ( num8 > 1.0)
                    num8 = 1f;
                  if ( num9 > 1.0)
                    num9 = 1f;
                  if ( num10 > 1.0)
                    num10 = 1f;
                  if (0.0 >  num8)
                    num1 = 0.0f;
                  if (0.0 >  num9)
                    num2 = 0.0f;
                  if (0.0 >  num10)
                    num3 = 0.0f;
                  if (isMilitia)
                  {
                    ref Graphics local7 = ref this.g2;
                    ref local8: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounterHigh;
                    rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                    let mut srcrect: &Rectangle = &rectangle1
                    rectangle2 = Rectangle::new(tx, ty, 38, 38);
                    let mut destrect: &Rectangle = &rectangle2
                    DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local9 = ref this.g2;
                    ref local10: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounterHigh;
                    rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                    let mut srcrect: &Rectangle = &rectangle1
                    rectangle2 = Rectangle::new(tx, ty, 38, 38);
                    let mut destrect: &Rectangle = &rectangle2
                    DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
                  }
                  if (flag3 & ShowAttacker)
                    DrawMod.DrawRectangle(ref this.g2, tx, ty, 38, 38, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                  if (flag3 & ShowAttacker)
                    DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 37, 37, 0, 0, 0, (int) byte.MaxValue);
                }
              }
              else if (!flag2)
              {
                ref Graphics local11 = ref this.g2;
                ref local12: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounter;
                rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(tx, ty, 38, 38);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect, destrect);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx, ty, 38, 38, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 37, 37, 0, 0, 0, (int) byte.MaxValue);
              }
              else
              {
                ref Graphics local13 = ref this.g2;
                ref local14: Bitmap = ref this.game.Data.RegimeObj[regimeNr].TempCounterHigh;
                rectangle1 = Rectangle::new(38 * landscapeType, 0, 38, 38);
                let mut srcrect: &Rectangle = &rectangle1
                rectangle2 = Rectangle::new(tx, ty, 38, 38);
                let mut destrect: &Rectangle = &rectangle2
                DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect, destrect);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx, ty, 38, 38, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                if (flag3 & ShowAttacker)
                  DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 37, 37, 0, 0, 0, (int) byte.MaxValue);
              }
            }
          }
          else
            goto label_382;
        }
        else if (OverruleHis > -1 & OverruleRegime > -1)
        {
          if (OverruleRegime > -1 && Information.IsNothing( this.game.Data.RegimeObj[OverruleRegime].TempCounter))
            this.game.Data.RegimeObj[OverruleRegime].DoTempCounter();
          if (OverruleRegime == 0)
            num4 = num4;
          let mut num11: i32 =  0;
          if (nr > -1)
            num11 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType;
          ref Graphics local15 = ref this.g2;
          ref local16: Bitmap = ref this.game.Data.RegimeObj[OverruleRegime].TempCounter;
          rectangle1 = Rectangle::new(38 * num11, 0, 38, 38);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(tx, ty, 38, 38);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect, destrect);
        }
        let mut num12: i32 =  0;
        if (isMilitia & this.game.Data.Turn == regimeNr & OverruleHis == -1)
          DrawMod.DrawTextColouredConsoleCenter(ref this.g2, "M", this.game.MarcFont5, tx + 32, ty + 1, Color.FromArgb(228, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
        else if (!isMilitia & this.game.Data.Turn == regimeNr & OverruleHis == -1)
        {
          counterString: String = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].CounterString;
          if (Operators.CompareString(Strings.Trim(counterString), Strings.Trim(Conversion.Val(counterString).ToString()), false) == 0)
          {
            let mut num13: i32 =  (int) Math.Round( this.g2.MeasureString(counterString, DrawMod.TGame.MarcFont5).Width);
            num12 = num13;
            DrawMod.DrawTextColouredConsoleCenter(ref this.g2, counterString, this.game.MarcFont5, tx + 41 - num13, ty + 1, Color.FromArgb(228, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
          }
        }
        if (nr > -1)
        {
          if (index1 > -1 & !this.game.Data.UnitObj[nr].IsHQ)
          {
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
            {
              let mut red: i32 =  this.game.Data.UnitObj[index1].Red;
              let mut green: i32 =  this.game.Data.UnitObj[index1].Green;
              let mut blue: i32 =  this.game.Data.UnitObj[index1].Blue;
              DrawMod.DrawBlockGradient2(ref this.g2, tx, ty + 25, 37, 12, Color.FromArgb(0, red, green, blue), Color.FromArgb(235, red, green, blue));
            }
          }
          else if (this.game.Data.UnitObj[nr].IsHQ & this.game.Data.UnitObj[nr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Type < 8)
          {
            let mut red: i32 =  this.game.Data.UnitObj[nr].Red;
            let mut green: i32 =  this.game.Data.UnitObj[nr].Green;
            let mut blue: i32 =  this.game.Data.UnitObj[nr].Blue;
            DrawMod.DrawBlockGradient2(ref this.g2, tx, ty + 1, 37, 35, Color.FromArgb(0, red, green, blue), Color.FromArgb(205, red, green, blue));
          }
        }
        else
          nr = nr;
        num14: i32;
        index3: i32;
        SizeF sizeF3;
        num15: i32;
        if (!mostlyHidden)
        {
          if (OverruleHis == -1)
          {
            let mut num16: i32 =  0;
            if (this.game.Data.UnitObj[nr].HQ == -1 & !this.game.Data.UnitObj[nr].IsHQ)
              num16 = 0;
            let mut num17: i32 =  -1;
            let mut num18: i32 =  -1;
            if (coordinate.x > 1)
            {
              if (!this.game.Data.UnitObj[nr].IsHQ)
              {
                let mut sfCount1: i32 =  this.game.Data.UnitObj[nr].SFCount;
                num19: i32;
                for (let mut index4: i32 =  0; index4 <= sfCount1; index4 += 1)
                {
                  let mut sf: i32 =  this.game.Data.UnitObj[nr].SFList[index4];
                  let mut index5: i32 =  this.game.Data.SFObj[sf].Type;
                  let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                  if (this.game.Data.UnitObj[nr].X > -1 && index5 > -1 && this.game.Data.SFTypeObj[index5].Theater != 1 & this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[nr].X, this.game.Data.UnitObj[nr].Y].LandscapeType].IsSea)
                    index5 = -1;
                  if (index5 > -1)
                  {
                    let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[index5].SymbolGroup;
                    let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[index5].SymbolWeight;
                    if (symbolGroup > -1)
                    {
                      numArray2[symbolGroup] = numArray2[symbolGroup] + symbolWeight * qty * 10;
                      if (this.game.Data.SFTypeObj[index5].CarryCap > 0)
                        numArray2[symbolGroup] = numArray2[symbolGroup] + 1;
                      if (numArray2[symbolGroup] > num18)
                      {
                        num18 = numArray2[symbolGroup];
                        num19 = symbolGroup;
                      }
                    }
                  }
                }
                if (num19 > -1 | this.game.Data.UnitObj[nr].Historical > -1 & this.game.EditObj.HideUnit == 2)
                {
                  let mut num20: i32 =  num19;
                  let mut sfTypeNr: i32 =  -1;
                  let mut num21: i32 =  0;
                  let mut index6: i32 =  -1;
                  let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
                  for (let mut index7: i32 =  0; index7 <= sfTypeCounter; index7 += 1)
                    numArray2[index7] = 0;
                  let mut sfCount2: i32 =  this.game.Data.UnitObj[nr].SFCount;
                  for (let mut index8: i32 =  0; index8 <= sfCount2; index8 += 1)
                  {
                    let mut sf: i32 =  this.game.Data.UnitObj[nr].SFList[index8];
                    let mut type: i32 =  this.game.Data.SFObj[sf].Type;
                    let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
                    let mut symbolGroup: i32 =  this.game.Data.SFTypeObj[type].SymbolGroup;
                    let mut symbolWeight: i32 =  this.game.Data.SFTypeObj[type].SymbolWeight;
                    if (symbolGroup == num20)
                    {
                      numArray2[type] = numArray2[type] + symbolWeight * qty * 10;
                      if (this.game.Data.SFTypeObj[type].CarryCap > 0)
                        numArray2[type] = numArray2[type] + 1;
                      if (numArray2[type] > num21)
                      {
                        num21 = numArray2[type];
                        sfTypeNr = type;
                        index6 = sf;
                      }
                    }
                  }
                  if (sfTypeNr > -1)
                  {
                    if (this.game.EditObj.HideUnit != 2 & sfTypeNr > -1 & this.game.Data.UnitObj[nr].Historical > -1 & this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[89] > 0)
                    {
                      let mut tv0: i32 =  this.game.Data.PeopleObj[this.game.Data.SFObj[index6].People].tv0;
                      if (this.slotCulture < 0)
                        this.slotCulture = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
                      let mut integer2: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.slotCulture].GetData(0, tv0, 1));
                      objBitmap: Bitmap;
                      if (!Information.IsNothing( this.game.Data.UnitObj[nr].tempSFTypeBitmap))
                      {
                        objBitmap = this.game.Data.UnitObj[nr].tempSFTypeBitmap;
                      }
                      else
                      {
                        objBitmap = this.DrawSFTypeGraphic(sfTypeNr, isMilitia, integer2, regimeNr, nr);
                        this.game.Data.UnitObj[nr].tempSFTypeBitmap = objBitmap;
                      }
                      if (!Information.IsNothing( objBitmap))
                      {
                        let mut num22: i32 =  0;
                        let mut num23: i32 =  0;
                        let mut w: i32 =  38;
                        let mut h: i32 =  32;
                        let mut width: i32 =  objBitmap.Width;
                        let mut height: i32 =  objBitmap.Height;
                        if (width > w | height > h)
                        {
                          if ( width /  w >  height /  h)
                          {
                            float num24 =  w /  width;
                            let mut num25: i32 =  (int) Math.Round( ( h -  height * num24));
                            num23 += (int) Math.Round( num25 / 2.0);
                            h -= num25;
                          }
                          else
                          {
                            float num26 =  h /  height;
                            let mut num27: i32 =  (int) Math.Round( ( w -  width * num26));
                            num22 += (int) Math.Round( num27 / 2.0);
                            w -= num27;
                          }
                          if (this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Mirror)
                          {
                            Matrix matrix = Matrix::new();
                            matrix.Scale(-1f, 1f);
                            if (Information.IsNothing( toG))
                              matrix.Translate( -(w + 0), 0.0f);
                            else
                              matrix.Translate( -(2 * (num22 + tx) + (w + 0)), 0.0f);
                            this.g2.Transform = matrix;
                          }
                          DrawMod.DrawScaled(ref this.g2, ref objBitmap, tx + num22, ty + num23, w, h);
                        }
                        else
                        {
                          if (this.game.Data.RegimeObj[this.game.Data.UnitObj[nr].Regime].Mirror)
                          {
                            Matrix matrix = Matrix::new();
                            matrix.Scale(-1f, 1f);
                            if (Information.IsNothing( toG))
                              matrix.Translate( -(w + 0), 0.0f);
                            else
                              matrix.Translate( -(2 * tx + (w + 0)), 0.0f);
                            this.g2.Transform = matrix;
                          }
                          DrawMod.DrawSimple(ref this.g2, ref objBitmap, tx + num22 + (int) Math.Round( (w - width) / 2.0), ty + num23 + (int) Math.Round( (h - height) / 2.0));
                        }
                        this.g2.ResetTransform();
                      }
                    }
                    else if (sfTypeNr > -1 | this.game.Data.UnitObj[nr].Historical > -1 & this.game.EditObj.HideUnit == 2 |  this.game.Data.RuleVar[344] == 0.0)
                    {
                      let mut nr1: i32 =  -1;
                      if (sfTypeNr > -1)
                        nr1 = this.game.Data.SFTypeObj[sfTypeNr].SymbolSpriteID;
                      let mut nr2: i32 =  -1;
                      if (regimeNr > -1 & sfTypeNr > -1)
                      {
                        if (this.game.Data.RegimeObj[regimeNr].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index9: i32 =  0; index9 <= extraCounter; index9 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index9] == this.game.Data.RegimeObj[regimeNr].ExtraGraphicUse)
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index9];
                          }
                        }
                        else if (this.game.Data.PeopleObj[integer1].ExtraGraphicUse > -1)
                        {
                          let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sfTypeNr].ExtraCounter;
                          for (let mut index10: i32 =  0; index10 <= extraCounter; index10 += 1)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].ExtraCode[index10] == this.game.Data.PeopleObj[integer1].ExtraGraphicUse)
                              nr1 = this.game.Data.SFTypeObj[sfTypeNr].ExtraSymbolSpriteID[index10];
                          }
                        }
                      }
                      num28: i32;
                      if (this.game.EditObj.HideUnit == 2 && this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
                      {
                        nr1 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx <= -1 ? -1 : this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx];
                        if (nr1 == -1 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter > 0 & this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= this.game.NATO.GetUpperBound(0))
                          nr1 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter];
                        if (nr1 > -1)
                        {
                          num28 = 8;
                          if (this.game.Data.UnitObj[nr].HistoricalSubPart > -1)
                          {
                            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                              nr2 = this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].DesignationSmall[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                            else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart] > 0)
                              nr2 = this.game.NATO[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Designation[this.game.Data.UnitObj[nr].HistoricalSubPart]];
                          }
                        }
                      }
                      num17 = nr1;
                      let mut regime2: i32 =  this.game.Data.UnitObj[nr].Regime;
                      if (regime2 > -1 & sfTypeNr > -1)
                      {
                        float num29 = 1f;
                        float num30 = 1f;
                        float num31 = 1f;
                        if (this.game.EditObj.HideUnit == 2)
                        {
                          num16 -= 3;
                          num28 -= 15;
                        }
                        if (this.game.Data.RegimeObj[regime2].Mirror & !(this.game.EditObj.HideUnit == 2 & this.game.Data.UnitObj[nr].Historical > -1))
                        {
                          Matrix matrix = Matrix::new();
                          matrix.Scale(-1f, 1f);
                          if (Information.IsNothing( toG))
                            matrix.Translate(-45f, 0.0f);
                          else
                            matrix.Translate( -(2 * tx + 45), 0.0f);
                          this.g2.Transform = matrix;
                          if (this.game.EditObj.PrefMinimalistCounter && this.game.Data.Product < 6)
                            num16 = 3;
                          if (this.game.Data.Product == 7)
                            num16 += 6;
                        }
                        if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[82] > 0 & this.game.EditObj.HideUnit != 2)
                          num28 = -2;
                        if ( num29 == 1.0 &  num30 == 1.0 &  num31 == 1.0)
                        {
                          if (this.game.Data.SFTypeObj[sfTypeNr].SFTypeVar[82] > 0 & nr1 > 0)
                          {
                            if (this.game.Data.SFTypeObj[sfTypeNr].artCode[0] < 1 | this.game.EditObj.HideUnit == 2)
                            {
                              ref Graphics local17 = ref this.g2;
                              bitmap = BitmapStore.GetBitmap(nr1);
                              ref local18: Bitmap = ref bitmap;
                              let mut x: i32 =  num16 + tx;
                              let mut y: i32 =  ty + num28;
                              DrawMod.DrawSimple(ref local17, ref local18, x, y);
                            }
                            else
                            {
                              ref Graphics local19 = ref this.g2;
                              bitmap = BitmapStore.GetBitmap(nr1);
                              ref local20: Bitmap = ref bitmap;
                              rectangle1 = Rectangle::new(0, 0, 38, 38);
                              let mut srcrect: &Rectangle = &rectangle1
                              rectangle2 = Rectangle::new(num16 + tx, ty + num28, 38, 38);
                              let mut destrect: &Rectangle = &rectangle2
                              double r =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[1] /  byte.MaxValue);
                              double g =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[2] /  byte.MaxValue);
                              double b =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[3] /  byte.MaxValue);
                              DrawMod.DrawSimplePart2ColouredNew(ref local19, ref local20, srcrect, destrect,  r,  g,  b, 1f);
                            }
                            if (this.game.Data.SFTypeObj[sfTypeNr].artCode[5] == 1 & this.game.EditObj.HideUnit != 2)
                            {
                              ref Graphics local21 = ref this.g2;
                              bitmap = BitmapStore.GetBitmap(this.game.Data.SFTypeObj[sfTypeNr].SymbolColBigSpriteID);
                              ref local22: Bitmap = ref bitmap;
                              rectangle1 = Rectangle::new(0, 0, 38, 38);
                              let mut srcrect: &Rectangle = &rectangle1
                              rectangle2 = Rectangle::new(num16 + tx, ty + num28, 38, 38);
                              let mut destrect: &Rectangle = &rectangle2
                              double r =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[6] /  byte.MaxValue);
                              double g =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[7] /  byte.MaxValue);
                              double b =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[8] /  byte.MaxValue);
                              double a =  ( this.game.Data.SFTypeObj[sfTypeNr].artCode[9] /  byte.MaxValue);
                              DrawMod.DrawSimplePart2ColouredNew(ref local21, ref local22, srcrect, destrect,  r,  g,  b,  a);
                            }
                          }
                          else if (nr1 > 0)
                          {
                            ref Graphics local23 = ref this.g2;
                            bitmap = BitmapStore.GetBitmap(nr1);
                            ref local24: Bitmap = ref bitmap;
                            let mut x: i32 =  num16 + tx;
                            let mut y: i32 =  ty + num28;
                            DrawMod.DrawSimple(ref local23, ref local24, x, y);
                          }
                          this.g2.ResetTransform();
                          if (nr2 > 0)
                          {
                            ref Graphics local25 = ref this.g2;
                            bitmap = BitmapStore.GetBitmap(nr2);
                            ref local26: Bitmap = ref bitmap;
                            let mut x: i32 =  num16 + tx;
                            let mut y: i32 =  ty + num28;
                            DrawMod.DrawSimple(ref local25, ref local26, x, y);
                          }
                        }
                        else
                        {
                          if (nr1 > 0)
                          {
                            ref Graphics local27 = ref this.g2;
                            bitmap = BitmapStore.GetBitmap(nr1);
                            ref local28: Bitmap = ref bitmap;
                            let mut x: i32 =  num16 + tx;
                            let mut y: i32 =  ty + num28;
                            double r =  num29 - 1.0;
                            double g =  num30 - 1.0;
                            double b =  num31 - 1.0;
                            DrawMod.Draw(ref local27, ref local28, x, y,  r,  g,  b, 1f);
                          }
                          this.g2.ResetTransform();
                          if (nr2 > 0)
                          {
                            ref Graphics local29 = ref this.g2;
                            bitmap = BitmapStore.GetBitmap(nr2);
                            ref local30: Bitmap = ref bitmap;
                            let mut x: i32 =  num16 + tx;
                            let mut y: i32 =  ty + num28;
                            double r =  num29 - 1.0;
                            double g =  num30 - 1.0;
                            double b =  num31 - 1.0;
                            DrawMod.Draw(ref local29, ref local30, x, y,  r,  g,  b, 1f);
                          }
                        }
                      }
                      else
                      {
                        if ( this.game.Data.RuleVar[847] == 1.0 & this.game.EditObj.HideUnit == 2 & this.game.Data.UnitObj[nr].Historical > -1)
                          num28 -= 8;
                        if (nr1 > 0)
                        {
                          ref Graphics local31 = ref this.g2;
                          bitmap = BitmapStore.GetBitmap(nr1);
                          ref local32: Bitmap = ref bitmap;
                          let mut x: i32 =  num16 + tx;
                          let mut y: i32 =  ty + num28;
                          DrawMod.DrawSimple(ref local31, ref local32, x, y);
                        }
                        this.g2.ResetTransform();
                        if (nr2 > 0)
                        {
                          ref Graphics local33 = ref this.g2;
                          bitmap = BitmapStore.GetBitmap(nr2);
                          ref local34: Bitmap = ref bitmap;
                          let mut x: i32 =  num16 + tx;
                          let mut y: i32 =  ty + num28;
                          DrawMod.DrawSimple(ref local33, ref local34, x, y);
                        }
                      }
                    }
                  }
                }
              }
              else
              {
                if (this.game.EditObj.HideUnit == 2 && this.game.Data.UnitObj[nr].Historical > -1 &  this.game.Data.RuleVar[344] == 1.0)
                  num14 = 8;
                if (this.game.Data.UnitObj[nr].IsHQ)
                  num14 = 0;
                index3 = -1;
                regime1 = this.game.Data.UnitObj[nr].Regime;
                if (this.game.Data.UnitObj[nr].Regime > -1)
                {
                  let mut hqSpriteNr: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr;
                  float num32 =  this.game.Data.RegimeObj[index2].Red / 256f;
                  float num33 =  this.game.Data.RegimeObj[index2].Green / 256f;
                  float num34 =  this.game.Data.RegimeObj[index2].Blue / 256f;
                  ref Graphics local35 = ref this.g2;
                  bitmap = BitmapStore.GetBitmap(hqSpriteNr);
                  ref local36: Bitmap = ref bitmap;
                  let mut x1: i32 =  tx - 4;
                  let mut y1: i32 =  ty + 2;
                  double r1 =  num32 - 1.0;
                  double g1 =  num33 - 1.0;
                  double b1 =  num34 - 1.0;
                  DrawMod.Draw(ref local35, ref local36, x1, y1,  r1,  g1,  b1, 1f);
                  let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[index2].HQSpriteNr2;
                  if (this.game.Data.UnitObj[nr].Historical <= -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].SmallGfx > -1 || this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].Counter <= 0)
                    ;
                  float num35;
                  float num36;
                  float num37;
                  if (!this.game.Data.RegimeObj[index2].HQSpriteOverrule)
                  {
                    num35 =  this.game.Data.RegimeObj[index2].Red3 / 256f;
                    num36 =  this.game.Data.RegimeObj[index2].Green3 / 256f;
                    num37 =  this.game.Data.RegimeObj[index2].Blue3 / 256f;
                  }
                  else
                  {
                    num35 = 1f;
                    num36 = 1f;
                    num37 = 1f;
                  }
                  ref Graphics local37 = ref this.g2;
                  bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
                  ref local38: Bitmap = ref bitmap;
                  let mut x2: i32 =  tx - 4;
                  let mut y2: i32 =  ty + 2;
                  double r2 =  num35 - 1.0;
                  double g2 =  num36 - 1.0;
                  double b2 =  num37 - 1.0;
                  DrawMod.Draw(ref local37, ref local38, x2, y2,  r2,  g2,  b2, 1f);
                  counterString: String = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[nr].Historical].CounterString;
                  str: String = Strings.UCase(Strings.Left(counterString, 1)) + Strings.Mid(counterString, 2);
                  Conversion.Str( Conversion.Val(str));
                  sizeF3 = this.g2.MeasureString(str, this.stdFont1);
                  let mut num38: i32 =  (int) Math.Round(20.0 -  (int) Math.Round( sizeF3.Width) / 2.0);
                  DrawMod.DrawTextColouredMarcCounter(ref this.g2, Strings.Trim(str), this.stdFont1, tx + num38, ty + 23, Color.White);
                }
              }
            }
          }
          else if (OverruleRegime > -1 & OverruleHis > -1 & OverrulePower == -9999 &  this.game.Data.RuleVar[344] == 1.0 & OverruleHis <= this.game.Data.HistoricalUnitObj.GetUpperBound(0))
          {
            let mut num39: i32 =  12;
            float a1;
            float a2;
            float a3;
            if (OverruleRegime > -1)
            {
              a1 =  this.game.Data.RegimeObj[OverruleRegime].Red3;
              a2 =  this.game.Data.RegimeObj[OverruleRegime].Green3;
              a3 =  this.game.Data.RegimeObj[OverruleRegime].Blue3;
            }
            else
            {
              a1 = 128f;
              a2 = 128f;
              a3 = 128f;
            }
            sizeF3 = this.g2.MeasureString("?", this.stdFont1);
            let mut num40: i32 =  (int) Math.Round(19.0 -  sizeF3.Width / 2.0);
            DrawMod.DrawTextColouredMarcCounter(ref this.g2, "?", this.stdFont1, tx + num40, ty + num39, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( a1), (int) Math.Round( a2), (int) Math.Round( a3)));
          }
          else if (OverruleRegime > -1 & OverruleHis > -1 & OverrulePower != -9999 &  this.game.Data.RuleVar[344] == 1.0 & OverruleHis <= this.game.Data.HistoricalUnitObj.GetUpperBound(0))
          {
            let mut num41: i32 =  -11;
            bool flag = false;
            hqSpriteNr: i32;
            float num42;
            float num43;
            float num44;
            if (this.game.Data.HistoricalUnitObj[OverruleHis].Type < 5)
            {
              if (this.game.Data.HistoricalUnitObj[OverruleHis].SmallGfx > -1)
                hqSpriteNr = this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[OverruleHis].SmallGfx];
              else if (this.game.Data.HistoricalUnitObj[OverruleHis].Counter > -1)
                hqSpriteNr = this.game.NATO[this.game.Data.HistoricalUnitObj[OverruleHis].Counter];
              if (this.game.Data.SFTypeObj[index3].SymbolOverrule)
              {
                num42 = 1f;
                num43 = 1f;
                num44 = 1f;
              }
              else
              {
                num42 = 1f;
                num43 = 1f;
                num44 = 1f;
              }
            }
            else
            {
              if (this.game.Data.HistoricalUnitObj[OverruleHis].SmallGfx > -1)
              {
                hqSpriteNr = this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[OverruleHis].SmallGfx];
              }
              else
              {
                hqSpriteNr = this.game.Data.RegimeObj[OverruleRegime].HQSpriteNr;
                num41 = 2;
                flag = true;
              }
              if (!this.game.Data.RegimeObj[OverruleRegime].HQSpriteOverrule)
              {
                num42 =  this.game.Data.RegimeObj[OverruleRegime].Red3 / 256f;
                num43 =  this.game.Data.RegimeObj[OverruleRegime].Green3 / 256f;
                num44 =  this.game.Data.RegimeObj[OverruleRegime].Blue3 / 256f;
              }
              else
              {
                num42 = 1f;
                num43 = 1f;
                num44 = 1f;
              }
            }
            let mut nr3: i32 =  -1;
            num45: i32;
            let mut num46: i32 =  num45 - 3;
            if ( num42 == 1.0 &  num43 == 1.0 &  num44 == 1.0)
            {
              if (flag)
              {
                float num47 =  this.game.Data.RegimeObj[OverruleRegime].Red / 256f;
                float num48 =  this.game.Data.RegimeObj[OverruleRegime].Green / 256f;
                float num49 =  this.game.Data.RegimeObj[OverruleRegime].Blue / 256f;
                ref Graphics local39 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(hqSpriteNr);
                ref local40: Bitmap = ref bitmap;
                let mut x3: i32 =  num46 + tx;
                let mut y3: i32 =  ty + num41;
                double r =  num47 - 1.0;
                double g =  num48 - 1.0;
                double b =  num49 - 1.0;
                DrawMod.Draw(ref local39, ref local40, x3, y3,  r,  g,  b, 1f);
                let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[OverruleRegime].HQSpriteNr2;
                num1 =  this.game.Data.RegimeObj[OverruleRegime].Red3 / 256f;
                num2 =  this.game.Data.RegimeObj[OverruleRegime].Green3 / 256f;
                num3 =  this.game.Data.RegimeObj[OverruleRegime].Blue3 / 256f;
                if (hqSpriteNr2 > -1)
                {
                  ref Graphics local41 = ref this.g2;
                  bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
                  ref local42: Bitmap = ref bitmap;
                  let mut x4: i32 =  num46 + tx;
                  let mut y4: i32 =  ty + num41;
                  DrawMod.DrawSimple(ref local41, ref local42, x4, y4);
                }
              }
              else
              {
                ref Graphics local43 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(hqSpriteNr);
                ref local44: Bitmap = ref bitmap;
                let mut x: i32 =  num46 + tx;
                let mut y: i32 =  ty + num41;
                DrawMod.DrawSimple(ref local43, ref local44, x, y);
              }
              this.g2.ResetTransform();
              if (nr3 > -1)
              {
                ref Graphics local45 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(nr3);
                ref local46: Bitmap = ref bitmap;
                let mut x: i32 =  num46 + tx;
                let mut y: i32 =  ty + num41;
                DrawMod.DrawSimple(ref local45, ref local46, x, y);
              }
            }
            else
            {
              if (flag)
              {
                float num50 =  this.game.Data.RegimeObj[OverruleRegime].Red / 256f;
                float num51 =  this.game.Data.RegimeObj[OverruleRegime].Green / 256f;
                float num52 =  this.game.Data.RegimeObj[OverruleRegime].Blue / 256f;
                ref Graphics local47 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(hqSpriteNr);
                ref local48: Bitmap = ref bitmap;
                let mut x5: i32 =  num46 + tx;
                let mut y5: i32 =  ty + num41;
                double r =  num50 - 1.0;
                double g =  num51 - 1.0;
                double b =  num52 - 1.0;
                DrawMod.Draw(ref local47, ref local48, x5, y5,  r,  g,  b, 1f);
                let mut hqSpriteNr2: i32 =  this.game.Data.RegimeObj[OverruleRegime].HQSpriteNr2;
                num42 =  this.game.Data.RegimeObj[OverruleRegime].Red3 / 256f;
                num43 =  this.game.Data.RegimeObj[OverruleRegime].Green3 / 256f;
                num44 =  this.game.Data.RegimeObj[OverruleRegime].Blue3 / 256f;
                if (hqSpriteNr2 > -1)
                {
                  ref Graphics local49 = ref this.g2;
                  bitmap = BitmapStore.GetBitmap(hqSpriteNr2);
                  ref local50: Bitmap = ref bitmap;
                  let mut x6: i32 =  num46 + tx;
                  let mut y6: i32 =  ty + num41;
                  DrawMod.DrawSimple(ref local49, ref local50, x6, y6);
                }
              }
              else
              {
                ref Graphics local51 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(hqSpriteNr);
                ref local52: Bitmap = ref bitmap;
                let mut x: i32 =  num46 + tx;
                let mut y: i32 =  ty + num41;
                double r =  num42 - 1.0;
                double g =  num43 - 1.0;
                double b =  num44 - 1.0;
                DrawMod.Draw(ref local51, ref local52, x, y,  r,  g,  b, 1f);
              }
              this.g2.ResetTransform();
              if (nr3 > -1)
              {
                ref Graphics local53 = ref this.g2;
                bitmap = BitmapStore.GetBitmap(nr3);
                ref local54: Bitmap = ref bitmap;
                let mut x: i32 =  num46 + tx;
                let mut y: i32 =  ty + num41;
                double r =  num42 - 1.0;
                double g =  num43 - 1.0;
                double b =  num44 - 1.0;
                DrawMod.Draw(ref local53, ref local54, x, y,  r,  g,  b, 1f);
              }
            }
            if (OverrulePower > -1 & OverruleRegime > -1)
            {
              float red3 =  this.game.Data.RegimeObj[OverruleRegime].Red3;
              float green3 =  this.game.Data.RegimeObj[OverruleRegime].Green3;
              float blue3 =  this.game.Data.RegimeObj[OverruleRegime].Blue3;
              let mut num53: i32 =  (int) Math.Round(19.0 -  this.g2.MeasureString(Strings.Trim(Conversion.Str( OverrulePower)), this.stdFont1).Width / 2.0);
              DrawMod.DrawTextColouredMarcCounter(ref this.g2, Strings.Trim(Conversion.Str( OverrulePower)), this.stdFont1, tx + num53, ty + 16 + 5, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( red3), (int) Math.Round( green3), (int) Math.Round( blue3)));
            }
          }
          else if (nr > -1)
          {
            num14 = 0;
            if (OverruleRegime == -1)
              OverruleRegime = this.game.Data.UnitObj[nr].Regime;
            num15 = this.game.Data.RegimeObj[OverruleRegime].HQSpriteNr;
            if (!this.game.Data.RegimeObj[OverruleRegime].HQSpriteOverrule)
            {
              num1 =  this.game.Data.RegimeObj[OverruleRegime].Red3 / 256f;
              num2 =  this.game.Data.RegimeObj[OverruleRegime].Green3 / 256f;
              num3 =  this.game.Data.RegimeObj[OverruleRegime].Blue3 / 256f;
            }
            else
            {
              num1 = 1f;
              num2 = 1f;
              num3 = 1f;
            }
            this.g2.ResetTransform();
          }
        }
        if (OverruleHis == -1 & !mostlyHidden)
        {
          if (!this.game.Data.UnitObj[nr].IsHQ)
          {
            regime1 = this.game.Data.UnitObj[nr].Regime;
            let mut num54: i32 =  coordinate.x != 2 ? ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, FullRecon: FullRecon, AbsPower: true)) : ( this.game.Data.RuleVar[847] != 1.0 ? this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y) : this.game.HandyFunctionsObj.GetPower(nr, this.game.Data.Turn, randomelement: coordinate.y, AbsPower: true));
            if (num54 > 999)
              num54 = 999;
            if (OverrulePower > -1)
              num54 = OverrulePower;
            else if (OverrulePower == -9999)
              num54 = -9999;
            let mut num55: i32 =  num54;
            if (OverruleHis == -1 & this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
            {
              float a4 =  byte.MaxValue;
              float maxValue =  byte.MaxValue;
              float a5 =  byte.MaxValue;
              let mut a6: i32 =  196;
              let mut a7: i32 =  92;
              let mut num56: i32 =  (int) Math.Round(Math.Floor( this.game.HandyFunctionsObj.GetLowestAp(nr) / 10.0));
              if (num56 > 10)
                num56 = 8;
              if (isMilitia & num56 > 5)
                num56 = 5;
              if ((int) Math.Round(10.0 -  num12 / 3.0) < num56)
                num56 = (int) Math.Round(10.0 -  num12 / 3.0);
              if (!this.game.Data.UnitObj[nr].DidAttack & !this.game.Data.UnitObj[nr].DidMove)
              {
                a6 = (int) byte.MaxValue;
                a7 = (int) byte.MaxValue;
                maxValue =  byte.MaxValue;
                a4 = 0.0f;
                a5 = 0.0f;
              }
              let mut num57: i32 =  num56;
              for (let mut index11: i32 =  1; index11 <= num57; index11 += 1)
              {
                DrawMod.DrawBlock(ref this.g2, tx + 10 + (index11 - 1) * 3, ty + 4, 2, 2, (int) Math.Round( a4), (int) Math.Round( maxValue), (int) Math.Round( a5), a6);
                DrawMod.DrawRectangle(ref this.g2, tx + 10 + (index11 - 1) * 3 - 1, ty + 4 - 1, 3, 3, 0, 0, 0, a7);
              }
              let mut averageRdn: i32 =  this.game.HandyFunctionsObj.GetAverageRdn(nr);
              let mut num58: i32 =  num55;
              this.game.HandyFunctionsObj.GetBreakPercent(nr);
              let mut num59: i32 =  (int) Math.Round( this.game.Data.RuleVar[307]);
              let mut num60: i32 =  this.game.HandyFunctionsObj.GetStartPower(nr);
              if (num60 == 0)
                num60 = num58;
              let mut num61: i32 =  (int) Math.Round(Conversion.Int( num58 /  num60 * 100.0));
              if ((int) Math.Round(34.0 - Conversion.Int( num58 /  num60 * 34.0)) < 0)
                num4 = 0;
              if (num58 > 0)
              {
                float a8;
                float a9;
                float a10;
                if (averageRdn >= 75)
                {
                  a8 = 0.0f;
                  a9 =  byte.MaxValue;
                  a10 = 0.0f;
                }
                else if (averageRdn >= 50)
                {
                  a8 =  byte.MaxValue;
                  a9 =  byte.MaxValue;
                  a10 = 0.0f;
                }
                else if (averageRdn >= 25)
                {
                  a8 = 0.0f;
                  a9 = 170f;
                  a10 =  byte.MaxValue;
                }
                else
                {
                  a8 =  byte.MaxValue;
                  a9 = 0.0f;
                  a10 = 0.0f;
                }
                let mut num62: i32 =  num61;
                if (num62 > 100)
                  num62 = 100;
                DrawMod.DrawBlock(ref this.g2, tx + 5, ty + 28, 28, 7, 0, 0, 0, 98);
                DrawMod.DrawBlock(ref this.g2, tx + 6, ty + 29, (int) Math.Round(26.0 * ( num62 / 100.0)), 5, (int) Math.Round( a8), (int) Math.Round( a9), (int) Math.Round( a10), 150);
              }
            }
            let mut Number: i32 =  num55;
            float red3 =  this.game.Data.RegimeObj[index2].Red3;
            float green3 =  this.game.Data.RegimeObj[index2].Green3;
            float blue3 =  this.game.Data.RegimeObj[index2].Blue3;
            if (OverruleHis > -1)
            {
              red3 =  this.game.Data.RegimeObj[OverruleRegime].Red3;
              green3 =  this.game.Data.RegimeObj[OverruleRegime].Green3;
              blue3 =  this.game.Data.RegimeObj[OverruleRegime].Blue3;
            }
            if (coordinate.x > 1 | OverruleHis > -1)
            {
              let mut num63: i32 =  OverruleHis != -1 ? OverruleHis : this.game.Data.UnitObj[nr].Historical;
              sizeF3 = this.g2.MeasureString(Strings.Trim(Conversion.Str( Number)), this.stdFont1);
              let mut num64: i32 =  (int) Math.Round(19.0 -  sizeF3.Width / 2.0);
              DrawMod.DrawTextColouredMarcCounter(ref this.g2, Strings.Trim(Conversion.Str( Number)), this.stdFont1, tx + num64, ty + 16 + 8, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( red3), (int) Math.Round( green3), (int) Math.Round( blue3)));
            }
            else
            {
              sizeF3 = this.g2.MeasureString("?", this.stdFont1);
              let mut num65: i32 =  (int) Math.Round(19.0 -  sizeF3.Width / 2.0);
              DrawMod.DrawTextColouredMarcCounter(ref this.g2, "?", this.stdFont1, tx + num65, ty + 16 + 8, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( red3), (int) Math.Round( green3), (int) Math.Round( blue3)));
            }
          }
          if (this.game.Data.UnitObj[nr].cycleOrder < 0L & this.game.Data.UnitObj[nr].Regime == this.game.EditObj.RealTurn)
          {
            DrawMod.DrawBlock(ref this.g2, tx + 2, ty + 8, 9, 9, 0, 0, 0, 200);
            DrawMod.DrawTextColouredMarcCenter(ref this.g2, "G", this.game.MarcFont5, tx + 7, ty + 7, Color.White);
          }
        }
        if (OverruleHis == -1 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
        {
          num15 = 0;
          if (this.game.EventRelatedObj.Helper_AirEnabled() & !this.game.EditObj.AIMoving && this.game.EditObj.Zoom >= 0 & OverruleHis == -1)
          {
            let mut historical: i32 =  this.game.Data.UnitObj[nr].Historical;
            this.slotAir = this.strId534slot;
            let mut num66: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(55);
            let mut num67: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(56);
            let mut num68: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(57);
            let mut numy: i32 =  -1;
            let mut num69: i32 =  -1;
            if (this.slotAir > -1 & num66 > 0)
            {
              let mut length: i32 =  this.game.Data.StringListObj[this.slotAir].Length;
              airRowNr: i32;
              for (let mut index12: i32 =  0; index12 <= length; index12 += 1)
              {
                if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 0])) == this.game.Data.RegimeObj[this.game.Data.Turn].id && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 1])) == this.game.Data.UnitObj[nr].X && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 2])) == this.game.Data.UnitObj[nr].Y && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 3])) == num67 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 4])) == num68 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 9])) >= 0)
                {
                  numy = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 8]));
                  num69 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[index12, 9]));
                  airRowNr = index12;
                  break;
                }
              }
              if (numy > -1)
              {
                letter: String = this.game.HandyFunctionsObj.CovertNumberToLetter(numy);
                color: Color = this.game.HandyFunctionsObj.Air_GetColor(airRowNr);
                let mut tcol: i32 =  0;
                let mut num70: i32 =  6;
                DrawMod.DrawBlock(ref this.g2, tx + num70, ty + 17, 11, 10, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                DrawMod.DrawTextCenterSmallLabel(ref this.g2, letter, this.game.MarcFont13, tx + num70 + 5, ty + 22, tcol);
              }
            }
          }
          float a11;
          float a12;
          float a13;
          if (this.game.Data.UnitObj[nr].SupplyInReq > 0)
          {
            let mut num71: i32 =  (int) Math.Round(Conversion.Int( (this.game.Data.UnitObj[nr].SupplyIn * 100) /  this.game.Data.UnitObj[nr].SupplyInReq));
            if (num71 >= 100)
            {
              a11 = 0.0f;
              a12 =  byte.MaxValue;
              a13 = 0.0f;
            }
            else if (num71 >= 66)
            {
              a11 =  byte.MaxValue;
              a12 =  byte.MaxValue;
              a13 = 0.0f;
            }
            else if (num71 >= 44)
            {
              a11 = 0.0f;
              a12 = 170f;
              a13 =  byte.MaxValue;
            }
            else if (num71 >= 22)
            {
              a11 =  byte.MaxValue;
              a12 = 0.0f;
              a13 = 0.0f;
            }
            else
            {
              a11 = 0.0f;
              a12 = 0.0f;
              a13 = 0.0f;
            }
          }
          else if (this.game.Data.UnitObj[nr].HQ == -1 | this.game.Data.UnitObj[nr].SupplyInReq == 0)
          {
            a11 = 0.0f;
            a12 =  byte.MaxValue;
            a13 = 0.0f;
          }
          else
          {
            a11 = 0.0f;
            a12 = 0.0f;
            a13 = 0.0f;
          }
          DrawMod.DrawBlock(ref this.g2, tx + 3, ty + 2, 4, 4, (int) Math.Round( a11), (int) Math.Round( a12), (int) Math.Round( a13), (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref this.g2, tx + 3, ty + 3, 4, 4, 0, 0, 0, 180);
        }
        if (OverruleHis == -1 &  this.game.Data.RuleVar[334] == 0.0 & !mostlyHidden && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
        {
          if (OverruleHis == -1 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime)
          {
            if (this.game.EditObj.ShowSameHistorical && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[nr].HQ > -1 & this.game.Data.UnitObj[nr].HQ == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ & !this.game.Data.UnitObj[nr].IsHQ | nr == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
            {
              float red =  this.game.Data.UnitObj[index1].Red;
              float green =  this.game.Data.UnitObj[index1].Green;
              float blue =  this.game.Data.UnitObj[index1].Blue;
              DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 35, 35, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 3);
            }
            if (this.game.EditObj.ShowUnderHQ && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.UnitObj[nr].HQ == this.game.EditObj.UnitSelected && this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
            {
              float red =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Red;
              float green =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Green;
              float blue =  this.game.Data.UnitObj[this.game.Data.UnitObj[nr].HQ].Blue;
              DrawMod.DrawRectangle(ref this.g2, tx + 1, ty + 1, 35, 35, (int) Math.Round( red), (int) Math.Round( green), (int) Math.Round( blue), 112, 3);
            }
          }
        }
        if (this.game.EditObj.UnitSelected == nr & this.game.EditObj.UnitSelected > -1)
        {
          DrawMod.DrawRectangle(ref this.g2, tx + 0, ty + 0, 38, 38, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref this.g2, tx - 1, ty - 1, 40, 40, 0, 0, 0, (int) byte.MaxValue);
          DrawMod.DrawRectangle(ref this.g2, tx + 2, ty + 2, 34, 34, 128, 128, 128, (int) byte.MaxValue);
        }
        if (OverruleHis == -1 &&  this.game.Data.RuleVar[983] > 0.0 && this.game.Data.Turn == this.game.Data.UnitObj[nr].Regime && !this.game.HandyFunctionsObj.CheckIfInCorrectFrontzone(nr, this.game.Data.UnitObj[nr].RealX(ref this.game), this.game.Data.UnitObj[nr].RealY(ref this.game)))
          DrawMod.DrawRectangle(ref this.g2, tx, ty - 1, 38, 39, (int) byte.MaxValue, 0, 0, 172, 2);
        this.game.EditObj.HideUnit = hideUnit;
        if (Information.IsNothing( toG))
          return this.tmpbmp2;
      }
label_382:
      bitmap1: Bitmap;
      return bitmap1;
    }

    pub DrawHistoryForce: Bitmap(regnr: i32, force: i32, sftype: i32)
    {
      SizeF sizeF = SizeF::new();
      int[] numArray = new int[1];
      numArray = this.game.Data.SFTypeCounter >= 101 ? new int[this.game.Data.SFTypeCounter + 1] : new int[101];
      this.g2 = Graphics.FromImage((Image) this.tmpbmp2);
      if (regnr <= -1)
      {
        ref Graphics local1 = ref this.g2;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.DEFAULTCOUNTER);
        ref local2: Bitmap = ref bitmap;
        DrawMod.DrawSimple(ref local1, ref local2, 0, 0);
      }
      else
      {
        if (Information.IsNothing( this.game.Data.RegimeObj[regnr].TempCounter))
          this.game.Data.RegimeObj[regnr].DoTempCounter();
        bool flag;
        if (!flag)
          DrawMod.DrawSimple(ref this.g2, ref this.game.Data.RegimeObj[regnr].TempCounter, 0, 0);
        else
          DrawMod.DrawSimple(ref this.g2, ref this.game.Data.RegimeObj[regnr].TempCounterHigh, 0, 0);
      }
      if (force != -9999)
      {
        if (sftype > -1)
        {
          let mut symbolSpriteId: i32 =  this.game.Data.SFTypeObj[sftype].SymbolSpriteID;
          if (regnr > -1 && this.game.Data.RegimeObj[regnr].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 =  this.game.Data.SFTypeObj[sftype].ExtraCounter;
            for (let mut index: i32 =  0; index <= extraCounter; index += 1)
            {
              if (this.game.Data.SFTypeObj[sftype].ExtraCode[index] == this.game.Data.RegimeObj[regnr].ExtraGraphicUse)
                symbolSpriteId = this.game.Data.SFTypeObj[sftype].ExtraSymbolSpriteID[index];
            }
          }
          if (regnr > -1)
          {
            if (this.game.Data.RegimeObj[regnr].Mirror)
            {
              Matrix matrix = Matrix::new();
              matrix.Scale(-1f, 1f);
              matrix.Translate(-38f, 0.0f);
              this.g2.Transform = matrix;
            }
            float num1;
            float num2;
            float num3;
            if (this.game.Data.SFTypeObj[sftype].SymbolOverrule)
            {
              num1 = 1f;
              num2 = 1f;
              num3 = 1f;
            }
            else
            {
              num1 = 1f;
              num2 = 1f;
              num3 = 1f;
            }
            ref Graphics local3 = ref this.g2;
            bitmap: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
            ref local4: Bitmap = ref bitmap;
            double r =  num1 - 1.0;
            double g =  num2 - 1.0;
            double b =  num3 - 1.0;
            DrawMod.Draw(ref local3, ref local4, -3, 0,  r,  g,  b, 1f);
            this.g2.ResetTransform();
          }
          else
          {
            ref Graphics local5 = ref this.g2;
            bitmap: Bitmap = BitmapStore.GetBitmap(symbolSpriteId);
            ref local6: Bitmap = ref bitmap;
            DrawMod.DrawSimple(ref local5, ref local6, -3, 0);
          }
        }
        if (force > -1 & regnr > -1)
        {
          float red2 =  this.game.Data.RegimeObj[regnr].Red2;
          float green2 =  this.game.Data.RegimeObj[regnr].Green2;
          float blue2 =  this.game.Data.RegimeObj[regnr].Blue2;
          let mut red: i32 =  0;
          let mut blue: i32 =  0;
          let mut green: i32 =  0;
          if ( red2 < 128.0 &  green2 < 128.0 &  blue2 < 128.0)
          {
            red = (int) byte.MaxValue;
            blue = (int) byte.MaxValue;
            green = (int) byte.MaxValue;
          }
          if (force > 9999)
            force = 9999;
          let mut Number: i32 =  force;
          let mut x: i32 =  (int) Math.Round(18.0 -  this.g2.MeasureString(Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel)).Width / 2.0);
          DrawMod.DrawTextColoured(ref this.g2, Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), x - 1, 23, Color.FromArgb((int) byte.MaxValue, red, green, blue));
          DrawMod.DrawTextColoured(ref this.g2, Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), x + 1, 23, Color.FromArgb((int) byte.MaxValue, red, green, blue));
          DrawMod.DrawTextColoured(ref this.g2, Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), x, 22, Color.FromArgb((int) byte.MaxValue, red, green, blue));
          DrawMod.DrawTextColoured(ref this.g2, Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), x, 24, Color.FromArgb((int) byte.MaxValue, red, green, blue));
          DrawMod.DrawTextColoured(ref this.g2, Strings.Trim(Conversion.Str( Number)), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), x, 23, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( red2), (int) Math.Round( green2), (int) Math.Round( blue2)));
        }
      }
      return this.tmpbmp2;
    }

    pub DrawBigCounter: Bitmap(regnr: i32)
    {
      SizeF sizeF = SizeF::new();
      int[] numArray = new int[1];
      bitmap1: Bitmap = new Bitmap(80, 80, PixelFormat.Format32bppPArgb);
      this.g2 = Graphics.FromImage((Image) bitmap1);
      this.g2.Clear(Color.Transparent);
      if (regnr == -1)
      {
        ref Graphics local1 = ref this.g2;
        bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.DEFAULTCOUNTERBIG);
        ref local2: Bitmap = ref bitmap2;
        DrawMod.DrawSimple(ref local1, ref local2, 0, 0);
      }
      else
      {
        if (Information.IsNothing( this.game.Data.RegimeObj[regnr].TempCounterBig))
          this.game.Data.RegimeObj[regnr].DoTempCounterBig();
        DrawMod.DrawSimple(ref this.g2, ref this.game.Data.RegimeObj[regnr].TempCounterBig, 0, 0);
      }
      this.g2.Dispose();
      return bitmap1;
    }

    pub fn ThreadBlock()
    {
      if (!DrawMod.TGame.se1ThreadBlock)
        return;
      while (DrawMod.TGame.se1ThreadBlock)
        Thread.Sleep(1);
      DrawMod.TGame.se1ThreadBlock = true;
    }

    pub fn ThreadRelease() => DrawMod.TGame.se1ThreadBlock = false;

    pub DrawHex: Bitmap(
      cx: i32,
      cy: i32,
      cmap: i32,
      bool InfoMode = false,
      bool NoShader = false,
      bool ispredrawing = false,
      Graphics tempg = null,
      let mut tx: i32 =  0,
      let mut ty: i32 =  0,
      let mut counteralpha: i32 =  255,
      let mut Zoom: i32 =  0,
      bool UseRegimeColoring = false,
      bool neverusehistory = false,
      bool combatSetup = false,
      ref gBitmap: Bitmap = null,
      ref bool tFromMapPopup = false)
    {
      this.ThreadBlock();
      int[] numArray1 = new int[7];
      int[] numArray2 = new int[7];
      Coordinate[] coordinateArray = new Coordinate[7];
      int[] numArray3 = new int[7];
      int[] numArray4 = new int[7];
      int[] numArray5 = new int[7];
      int[] numArray6 = new int[7];
      int[] numArray7 = new int[100];
      int[] numArray8 = new int[100];
      int[] numArray9 = new int[100];
      bool[] flagArray1 = new bool[100];
      bool[] flagArray2 = new bool[100];
      int[] numArray10 = new int[100];
      int[] numArray11 = new int[100];
      numArray12: Vec<i32> = new int[100, 6];
      SizeF sizeF1 = SizeF::new();
      double a1 =  (cy * this.game.Data.MapObj[cmap].MapWidth + cx) * 100.423 +  cx;
      double a2 =  ((cy + 1) * this.game.Data.MapObj[cmap].MapWidth + cx) * 100.423 * ( cx /  Math.Max(1, cy + 1));
      int[] numArray13 = new int[7];
      int[] numArray14 = new int[7];
      int[] numArray15 = new int[7];
      int[] numArray16 = new int[7];
      switch (Zoom)
      {
        case -1:
          numArray13[0] = 8;
          numArray14[0] = 4;
          numArray15[0] = 16;
          numArray16[0] = 16;
          numArray13[1] = 4;
          numArray14[1] = 0;
          numArray15[1] = 20;
          numArray16[1] = 4;
          numArray13[2] = 24;
          numArray14[2] = 0;
          numArray15[2] = 8;
          numArray16[2] = 12;
          numArray13[3] = 24;
          numArray14[3] = 12;
          numArray15[3] = 8;
          numArray16[3] = 12;
          numArray13[4] = 4;
          numArray14[4] = 22;
          numArray15[4] = 20;
          numArray16[4] = 2;
          numArray13[5] = 0;
          numArray14[5] = 12;
          numArray15[5] = 8;
          numArray16[5] = 12;
          numArray13[6] = 0;
          numArray14[6] = 0;
          numArray15[6] = 8;
          numArray16[6] = 12;
          break;
        case 0:
          numArray13[0] = 16;
          numArray14[0] = 8;
          numArray15[0] = 32;
          numArray16[0] = 32;
          numArray13[1] = 8;
          numArray14[1] = 0;
          numArray15[1] = 40;
          numArray16[1] = 4;
          numArray13[2] = 48;
          numArray14[2] = 0;
          numArray15[2] = 16;
          numArray16[2] = 24;
          numArray13[3] = 48;
          numArray14[3] = 24;
          numArray15[3] = 16;
          numArray16[3] = 24;
          numArray13[4] = 8;
          numArray14[4] = 44;
          numArray15[4] = 40;
          numArray16[4] = 4;
          numArray13[5] = 0;
          numArray14[5] = 24;
          numArray15[5] = 16;
          numArray16[5] = 24;
          numArray13[6] = 0;
          numArray14[6] = 0;
          numArray15[6] = 16;
          numArray16[6] = 24;
          break;
        case 1:
          numArray13[0] = 32;
          numArray14[0] = 16;
          numArray15[0] = 64;
          numArray16[0] = 64;
          numArray13[1] = 16;
          numArray14[1] = 0;
          numArray15[1] = 96;
          numArray16[1] = 8;
          numArray13[2] = 96;
          numArray14[2] = 0;
          numArray15[2] = 32;
          numArray16[2] = 48;
          numArray13[3] = 96;
          numArray14[3] = 48;
          numArray15[3] = 32;
          numArray16[3] = 48;
          numArray13[4] = 16;
          numArray14[4] = 88;
          numArray15[4] = 96;
          numArray16[4] = 8;
          numArray13[5] = 0;
          numArray14[5] = 48;
          numArray15[5] = 32;
          numArray16[5] = 48;
          numArray13[6] = 0;
          numArray14[6] = 0;
          numArray15[6] = 32;
          numArray16[6] = 48;
          break;
      }
      if (this.strId123slot < 1 | this.strId534slot < 1 | this.strId143slot < 1 | this.strId275slot < 1 | this.strId288slot < 1)
      {
        this.strId123slot = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
        this.strId534slot = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 534, 0, 0));
        this.strId143slot = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
        this.strId275slot = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 275, 0, 0));
        this.strId288slot = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 288, 0, 0));
      }
      index1: i32;
      index2: i32;
      if (!this.game.EditObj.se1_map_data3cache_set)
      {
        try
        {
          this.game.EditObj.se1_map_data3cache_set = true;
          data1: DataClass = this.game.Data;
          str1: String = "HeightMap";
          ref local1: String = ref str1;
          this.cache_t6 = data1.FindLibVar(ref local1, "SE_Random");
          data2: DataClass = this.game.Data;
          str2: String = "artifactDiscovered";
          ref local2: String = ref str2;
          this.cache_tad = data2.FindLibVar(ref local2, "SE_Data");
          data3: DataClass = this.game.Data;
          str3: String = "artifactType";
          ref local3: String = ref str3;
          this.cache_tat = data3.FindLibVar(ref local3, "SE_Data");
          data4: DataClass = this.game.Data;
          str3 = "rad";
          ref local4: String = ref str3;
          this.cache_rad = data4.FindLibVar(ref local4, "SE_Data");
          let mut num1: i32 =  Math.Max(0, this.game.Data.StringListObj[this.strId143slot].GetHighestValue(0)) + 20;
          let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
          for (index1 = 0; index1 <= regimeCounter; index1 += 1)
          {
            if (this.game.Data.RegimeObj[index1].id > num1)
              num1 = this.game.Data.RegimeObj[index1].id;
          }
          let mut num2: i32 =  Math.Max(0, this.game.Data.StringListObj[this.strId123slot].GetHighestValue(0)) + 20;
          this.cacheDipClear = new int[num1 + 1, num1 + 1];
          this.cacheZoneRecon = new int[num1 + 1, num2 + 1];
          let mut length1: i32 =  this.game.Data.StringListObj[this.strId275slot].Length;
          for (index1 = 0; index1 <= length1; index1 += 1)
          {
            if (Operators.CompareString(this.game.Data.StringListObj[this.strId275slot].Data[index1, 2].ToLower(), "dipclear", false) == 0)
            {
              index2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId275slot].Data[index1, 0]));
              let mut index3: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId275slot].Data[index1, 1]));
              let mut num3: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId275slot].Data[index1, 3]));
              if (index2 <= num1 & index3 <= num1)
                this.cacheDipClear[index2, index3] = num3;
            }
          }
          let mut length2: i32 =  this.game.Data.StringListObj[this.strId288slot].Length;
          for (index1 = 0; index1 <= length2; index1 += 1)
          {
            if (Operators.CompareString(this.game.Data.StringListObj[this.strId288slot].Data[index1, 2].ToLower(), "recon", false) == 0)
            {
              index2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId288slot].Data[index1, 0]));
              let mut index4: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId288slot].Data[index1, 1]));
              let mut num4: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId288slot].Data[index1, 3]));
              if (index2 <= num1 & index4 <= num2)
                this.cacheZoneRecon[index2, index4] = num4;
            }
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          this.game.EditObj.se1_map_data3cache_set = false;
          ProjectData.ClearProjectError();
        }
      }
      let mut num5: i32 =  this.cache_t6;
      if (!this.game.EditObj.AIMoving && this.game.Data.Turn > -1 && !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        this.game.EditObj.RealRound = this.game.Data.Round;
        this.game.EditObj.RealTurn = this.game.Data.Turn;
      }
      if (Information.IsNothing( this.tmpbmp3))
      {
        this.tmpbmp3 = new Bitmap(64, 48, PixelFormat.Format32bppPArgb);
        this.tmpbmp3.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      num6: i32;
      num7: i32;
      switch (Zoom)
      {
        case -1:
          num6 = 32;
          num7 = 24;
          break;
        case 0:
          num6 = 64;
          num7 = 48;
          break;
        default:
          num6 = 128;
          num7 = 96;
          break;
      }
      Graphics toG;
      if (Information.IsNothing( tempg) | Information.IsNothing( this.g3))
      {
        Graphics g3 = this.g3;
        toG = Graphics.FromImage((Image) this.tmpbmp3);
        toG.Clear(Color.FromArgb(0, 0, 0, 0));
      }
      else
        toG = tempg;
      if (this.game.Data.Product >= 7)
      {
        toG.InterpolationMode = InterpolationMode.NearestNeighbor;
        toG.TextRenderingHint = TextRenderingHint.SingleBitPerPixelGridFit;
        toG.PixelOffsetMode = PixelOffsetMode.None;
      }
      if (this.game.Data.Product >= 7)
        toG.CompositingQuality = CompositingQuality.HighSpeed;
      let mut num8: i32 =  0;
      let mut index5: i32 =  this.game.EditObj.RealTurn;
      if (this.game.Data.Round == 0)
        index5 = -1;
      bool flag1;
      if (this.game.EditObj.OrderType == 26)
      {
        num8 = 1;
        flag1 = true;
      }
      if (this.game.EditObj.AIMoving)
      {
        flag1 = true;
        num8 = 1;
        if (this.game.EditObj.HumanPlayer > -1 & this.game.EditObj.AIMoving)
          index5 = this.game.EditObj.HumanPlayer;
      }
      if (this.game.EditObj.RealRound == 0 & !this.game.EditObj.inRandomScreen)
        num8 = 1;
      else if (index5 > -1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 1 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5) > -1)
        num8 = 1;
      if (this.game.EditObj.OrderType == 38)
        num8 = 1;
      if (index5 != 2)
        index5 = index5;
      let mut num9: i32 =  0;
      if (this.game.EditObj.RealRound > 0 | this.game.EditObj.inRandomScreen)
      {
        if (index5 > -1)
        {
          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0)
            num9 = 1;
        }
        else
          num9 = 0;
      }
      if (!this.game.Data.ShrowdOn | num9 == 1)
        num8 = 1;
      bool flag2 = false;
      let mut num10: i32 =  0;
      bool flag3;
      if (this.game.EditObj.OrderType == 1)
      {
        num10 = 1;
        flag3 = true;
      }
      if (this.game.EditObj.OrderType == 48)
        num10 = 1;
      if (this.game.EditObj.OrderType == 18)
        num10 = 1;
      if (this.game.EditObj.OrderType == 36)
        num10 = 1;
      if (this.game.EditObj.OrderType == 11)
        num10 = 1;
      if (this.game.EditObj.OrderType == 14)
        num10 = 1;
      if (this.game.EditObj.OrderType == 33)
        num10 = 1;
      if (this.game.EditObj.OrderType == 55)
        num10 = 1;
      if (this.game.EditObj.RealRound > 0 & this.game.EditObj.RealTurn > -1 & !this.game.EditObj.AIMoving && !this.game.Data.RegimeObj[this.game.EditObj.RealTurn].AI && this.game.EditObj.OrderType == 0 & this.game.EditObj.ShowAirRange & this.game.EditObj.UnitSelected > -1)
      {
        if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
          num10 = 1;
        else if (this.game.HandyFunctionsObj.HasUnitArtSF(this.game.EditObj.UnitSelected, this.game.Data))
          num10 = 1;
      }
      if (this.game.Data.Product >= 7 && Information.IsNothing( this.game.EditObj.TempValueSpecial))
        this.game.HandyFunctionsObj.RedimTempValueSpecial(0);
      bool flag4;
      if (num10 == 1 & !this.game.EditObj.AIMoving)
      {
        if ((this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48) & this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
        {
          flag2 = true;
          if (this.game.EditObj.OrderUnit > -1 &  this.game.Data.RuleVar[983] > 0.0)
            flag4 = !this.game.HandyFunctionsObj.CheckIfInCorrectFrontzone(this.game.EditObj.OrderUnit, cx, cy);
          if (this.game.Data.Product >= 7)
          {
            if (Information.IsNothing( this.game.EditObj.TempValueSpecial))
              this.game.HandyFunctionsObj.RedimTempValueSpecial(0);
            if (this.game.EditObj.TempValueSpecial[cmap].Value[cx, cy] == 2)
              flag4 = true;
          }
        }
        if (this.game.EditObj.OrderType == 18)
        {
          if (this.game.EditObj.OrderUnit > -1)
          {
            let mut unitWeight: i32 =  this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.OrderUnit);
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical].Type == 8)
            {
              let mut counter: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].items.list.Counter;
              for (index1 = 0; index1 <= counter; index1 += 1)
              {
                index2 = this.game.HandyFunctionsObj.GetStringListByID((int) Math.Round( this.game.Data.RuleVar[404]));
                let mut integer: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[index2].GetData(0, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].items.list.Id[index1], 3));
                unitWeight += integer * this.game.Data.UnitObj[this.game.EditObj.OrderUnit].items.list.Weight[index1];
              }
            }
            if (this.game.EditObj.TempValue[cmap].Value[cx, cy] >= unitWeight)
              flag2 = true;
          }
          else if (this.game.EditObj.TempValue[cmap].Value[cx, cy] <= 0)
            ;
        }
        if (this.game.EditObj.OrderType == 36 && this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
          flag2 = true;
        if (this.game.EditObj.OrderType == 11 && this.game.EditObj.OrderUnit > -1 && this.game.EditObj.TempValue[cmap].Value[cx, cy] > 0 & this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
          flag2 = true;
        if (this.game.EditObj.OrderType == 14 && this.game.EditObj.OrderUnit > -1 && this.game.EditObj.TempValue[cmap].Value[cx, cy] > 0 & this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
          flag2 = true;
        if (this.game.EditObj.OrderType == 33 && this.game.EditObj.OrderUnit > -1 && this.game.EditObj.TempValue[cmap].Value[cx, cy] > 0 & this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
          flag2 = true;
        if (this.game.EditObj.OrderType == 55 && this.game.EditObj.OrderUnit > -1 && this.game.EditObj.TempValue[cmap].Value[cx, cy] > 0 & this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999)
          flag2 = true;
      }
      if (this.game.EditObj.AreaSlot > -1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].AreaCode[this.game.EditObj.AreaSlot] == this.game.EditObj.AreaValue)
        flag2 = true;
      if (neverusehistory)
        flag1 = false;
      if (flag1)
        flag2 = false;
      bitmap1: Bitmap;
      index6: i32;
      index7: i32;
      nr1: i32;
      float a3;
      float a4;
      float a5;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (num8 == 1 && ((!flag1 ? 1 : 0) | 1) == 0 && this.game.EditObj.HisOwner[cmap].Value[cx, cy] < -1 && this.game.EditObj.HisOwner[cmap].Value[cx, cy] == -2)
      {
        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5) == -1)
        {
          ref Graphics local5 = ref toG;
          bitmap1 = BitmapStore.GetBitmap(this.game.NOHEX, Zoom);
          ref local6: Bitmap = ref bitmap1;
          let mut x: i32 =  tx;
          let mut y: i32 =  ty;
          DrawMod.DrawSimple(ref local5, ref local6, x, y);
          num8 = 0;
        }
        else
        {
          index6 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
          index7 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastSpr(index5);
          if (!this.game.Data.LandscapeTypeObj[index6].Transparent)
          {
            if (index6 > -1)
              nr1 = this.game.Data.LandscapeTypeObj[index6].PreHexPicID;
            let mut num11: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
            let mut num12: i32 =  0;
            if (num11 > 1)
              num12 = new Random((int) Math.Round(a1)).Next(0, num11 - 1);
            if ((0 & (UseRegimeColoring ? 1 : 0) & (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime > -1 ? 1 : 0) & (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime != index5 ? 1 : 0)) != 0)
            {
              let mut regime: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime;
              a3 =  (0.3 +  this.game.Data.RegimeObj[regime].Red / 1024.0);
              a4 =  (0.3 +  this.game.Data.RegimeObj[regime].Green / 1024.0);
              a5 =  (0.3 +  this.game.Data.RegimeObj[regime].Blue / 1024.0);
              float num13 = 1f;
              ref Graphics local7 = ref toG;
              bitmap2: Bitmap = BitmapStore.GetBitmap(nr1, Zoom);
              ref local8: Bitmap = ref bitmap2;
              rectangle1 = Rectangle::new(num12 * num6, 0, num6, num7);
              let mut srcrect: &Rectangle = &rectangle1
              rectangle2 = Rectangle::new(tx, ty, num6, num7);
              let mut destrect: &Rectangle = &rectangle2
              double r =  a3;
              double g =  a4;
              double b =  a5;
              double a6 =  num13;
              DrawMod.DrawSimplePart2ColouredOld(ref local7, ref local8, srcrect, destrect,  r,  g,  b,  a6);
            }
            else
            {
              ref Graphics local9 = ref toG;
              bitmap3: Bitmap = BitmapStore.GetBitmap(nr1, Zoom);
              ref local10: Bitmap = ref bitmap3;
              rectangle2 = Rectangle::new(num12 * num6, 0, num6, num7);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, num6, num7);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect, destrect);
            }
            index2 = (int) Math.Round( BitmapStore.GetWidth(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID[index7], Zoom) /  num6);
            let mut num14: i32 =  0;
            if (index2 > 1)
              num14 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
            ref Graphics local11 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID[index7], Zoom);
            ref local12: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num14 * num6, 0, num6, num7);
            let mut srcrect1: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, num6, num7);
            let mut destrect1: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect1, destrect1);
            if (this.game.Data.LandscapeTypeObj[index6].BasicSpriteID2[index7] > 0 & this.game.Data.LandscapeTypeObj[index6].PlotLast[index7])
            {
              ref Graphics local13 = ref toG;
              bitmap1 = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID2[index7], Zoom);
              ref local14: Bitmap = ref bitmap1;
              rectangle2 = Rectangle::new(num14 * num6, 0, num6, num7);
              let mut srcrect2: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, num6, num7);
              let mut destrect2: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect2, destrect2);
            }
            ref Graphics local15 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(this.game.SHADEDHEX, Zoom);
            ref local16: Bitmap = ref bitmap1;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local15, ref local16, x, y);
          }
          num8 = 0;
        }
      }
      index8: i32;
      Coordinate coordinate1;
      num15: i32;
      Coordinate coordinate2;
      bitmap4: Bitmap;
      num16: i32;
      index9: i32;
      index10: i32;
      if (num8 == 1)
      {
        let mut lt1: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
        if (index5 > -1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
          lt1 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
        if (lt1 > -1)
        {
          nr1 = this.game.Data.LandscapeTypeObj[lt1].PreHexPicID;
          if ( this.game.Data.RuleVar[998] == 1.0 & this.game.Data.LandscapeTypeObj[lt1].UsePreHexTexture)
          {
            let mut preHexTextureId: i32 =  this.game.Data.LandscapeTypeObj[lt1].PreHexTextureID;
            if (Zoom == 0)
            {
              BitmapData bitmapdata = this.tempHexMed.LockBits(Rectangle::new(0, 0, 64, 48), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
              IntPtr scan0 = bitmapdata.Scan0;
              let mut num17: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexMed.Height;
              byte[] numArray17 = new byte[num17 - 1 + 1];
              Marshal.Copy(scan0, numArray17, 0, num17);
              let mut num18: i32 =  (cx + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (cy + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
              Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheMed, num18 * num17, (Array) numArray17, 0, num17);
              let mut index11: i32 =  0;
              let mut num19: i32 =  numArray17.Length - 1;
              for (let mut index12: i32 =  3; index12 <= num19; index12 += 4)
              {
                numArray17[index12] = BitmapStore.simpleByteCacheObj[this.game.WHITEHEX].singleCacheMed[index11];
                index11 += 1;
              }
              Marshal.Copy(numArray17, 0, scan0, num17);
              this.tempHexMed.UnlockBits(bitmapdata);
              DrawMod.DrawSimple(ref toG, ref this.tempHexMed, tx, ty);
            }
            if (Zoom == 1)
            {
              BitmapData bitmapdata = this.tempHexBig.LockBits(Rectangle::new(0, 0, 128, 96), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
              IntPtr scan0 = bitmapdata.Scan0;
              let mut num20: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexBig.Height;
              byte[] numArray18 = new byte[num20 - 1 + 1];
              Marshal.Copy(scan0, numArray18, 0, num20);
              let mut num21: i32 =  (cx + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (cy + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
              Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheBig, num21 * num20, (Array) numArray18, 0, num20);
              let mut index13: i32 =  0;
              let mut num22: i32 =  numArray18.Length - 1;
              for (let mut index14: i32 =  3; index14 <= num22; index14 += 4)
              {
                numArray18[index14] = BitmapStore.simpleByteCacheObj[this.game.WHITEHEX].singleCacheBig[index13];
                index13 += 1;
              }
              Marshal.Copy(numArray18, 0, scan0, num20);
              this.tempHexBig.UnlockBits(bitmapdata);
              DrawMod.DrawSimple(ref toG, ref this.tempHexBig, tx, ty);
            }
            if (Zoom == -1)
            {
              BitmapData bitmapdata = this.tempHexSmall.LockBits(Rectangle::new(0, 0, 32, 24), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
              IntPtr scan0 = bitmapdata.Scan0;
              let mut num23: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexSmall.Height;
              byte[] numArray19 = new byte[num23 - 1 + 1];
              Marshal.Copy(scan0, numArray19, 0, num23);
              let mut num24: i32 =  (cx + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (cy + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
              Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheSmall, num24 * num23, (Array) numArray19, 0, num23);
              let mut index15: i32 =  0;
              let mut num25: i32 =  numArray19.Length - 1;
              for (let mut index16: i32 =  3; index16 <= num25; index16 += 4)
              {
                numArray19[index16] = BitmapStore.simpleByteCacheObj[this.game.WHITEHEX].singleCacheSmall[index15];
                index15 += 1;
              }
              Marshal.Copy(numArray19, 0, scan0, num23);
              this.tempHexSmall.UnlockBits(bitmapdata);
              DrawMod.DrawSimple(ref toG, ref this.tempHexSmall, tx, ty);
            }
          }
          if ( this.game.Data.RuleVar[998] < 1.0 | this.game.Data.LandscapeTypeObj[lt1].OverridesZ < 899 & !this.game.Data.LandscapeTypeObj[lt1].UsePreHexTexture & !this.game.Data.LandscapeTypeObj[lt1].UsePreHexTextureAndRegularPreHex && !this.game.Data.LandscapeTypeObj[lt1].Transparent)
          {
            index2 = (int) Math.Round( BitmapStore.GetBitmap(nr1, Zoom).Width /  num6);
            let mut num26: i32 =  0;
            if (index2 > 1)
              num26 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
            ref Graphics local17 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
            ref local18: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num26 * num6, 0, num6, num7);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, num6, num7);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect, destrect);
          }
          int[] numArray20 = new int[7];
          int[] numArray21 = new int[7];
          let mut tfacing1: i32 =  1;
          index17: i32;
          do
          {
            coordinateArray[tfacing1] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing1);
            numArray1[tfacing1] = -1;
            if (coordinateArray[tfacing1].onmap)
            {
              index2 = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing1].x, coordinateArray[tfacing1].y].LandscapeType;
              if (index5 > -1)
              {
                if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing1].x, coordinateArray[tfacing1].y].get_SeeNow(index5) == 0)
                  index2 = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing1].x, coordinateArray[tfacing1].y].get_LastLT(index5);
                if (index2 == -1)
                  index2 = lt1;
              }
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[tfacing1 - 1] == 5)
                index2 = lt1;
              let mut num27: i32 =  tfacing1 + 3;
              if (num27 > 6)
                num27 -= 6;
              if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing1].x, coordinateArray[tfacing1].y].RiverType[num27 - 1] == 5)
                index2 = lt1;
              let mut preHexBorder: i32 =  this.game.Data.LandscapeTypeObj[index2].PreHexBorder;
              if (cx == 15 & cy == 6)
                index2 = index2;
              if (preHexBorder > -1 & preHexBorder != this.game.Data.LandscapeTypeObj[lt1].PreHexBorder && !this.game.Data.LandscapeTypeObj[preHexBorder].Transparent)
              {
                if (this.game.Data.LandscapeTypeObj[lt1].PreHexBorder > -1 | this.game.Data.LandscapeTypeObj[index2].PreHexBorder > -1)
                {
                  let mut num28: i32 =  0;
                  let mut num29: i32 =  0;
                  if (!this.game.Data.LandscapeTypeObj[index2].usePreHexBorderOwnZ | this.game.Data.LandscapeTypeObj[index2].OverridesZ > this.game.Data.LandscapeTypeObj[lt1].OverridesZ)
                    num28 = 1;
                  if (this.game.Data.LandscapeTypeObj[lt1].PreHexBorder > -1 && this.game.Data.LandscapeTypeObj[preHexBorder].OverridesZ > this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeObj[lt1].PreHexBorder].OverridesZ && this.game.Data.LandscapeTypeObj[index2].CheckOverride(lt1))
                    num28 = 1;
                  if (num28 == 1)
                  {
                    if (this.game.Data.LandscapeTypeObj[lt1].PreHexBorder > -1)
                    {
                      if (this.game.Data.LandscapeTypeObj[preHexBorder].OverridesZ > this.game.Data.LandscapeTypeObj[this.game.Data.LandscapeTypeObj[lt1].PreHexBorder].OverridesZ)
                        num29 = 1;
                    }
                    else
                      num29 = 1;
                    if (num29 == 1)
                    {
                      numArray1[tfacing1] = preHexBorder;
                      index17 += 1;
                      numArray20[index17] = tfacing1;
                      numArray21[index17] = this.game.Data.LandscapeTypeObj[preHexBorder].OverridesZ;
                    }
                  }
                }
                else if (!(this.game.Data.LandscapeTypeObj[lt1].IsSea & !this.game.Data.LandscapeTypeObj[lt1].Interior) && this.game.Data.LandscapeTypeObj[preHexBorder].OverridesZ > this.game.Data.LandscapeTypeObj[lt1].OverridesZ | !this.game.Data.LandscapeTypeObj[preHexBorder].IsSea & this.game.Data.LandscapeTypeObj[lt1].IsSea & this.game.Data.LandscapeTypeObj[lt1].Interior)
                {
                  numArray1[tfacing1] = preHexBorder;
                  index17 += 1;
                  numArray20[index17] = tfacing1;
                  numArray21[index17] = this.game.Data.LandscapeTypeObj[preHexBorder].OverridesZ;
                }
              }
            }
            tfacing1 += 1;
          }
          while (tfacing1 <= 6);
          let mut num30: i32 =  1;
          while (num30 == 1)
          {
            num30 = 0;
            let mut num31: i32 =  index17 - 1;
            for (let mut index18: i32 =  1; index18 <= num31; index18 += 1)
            {
              if (numArray21[index18] > numArray21[index18 + 1])
              {
                index2 = numArray21[index18];
                let mut num32: i32 =  numArray20[index18];
                numArray21[index18] = numArray21[index18 + 1];
                numArray20[index18] = numArray20[index18 + 1];
                numArray21[index18 + 1] = index2;
                numArray20[index18 + 1] = num32;
                num30 = 1;
              }
            }
          }
          let mut num33: i32 =  index17;
          index19: i32;
          num34: i32;
          for (index19 = 1; index19 <= num33; index19 += 1)
          {
            let mut index20: i32 =  numArray20[index19];
            if (numArray1[index20] > -1)
            {
              index2 = numArray1[index20];
              num34 = -1;
              int[] numArray22 = new int[7];
              let mut index21: i32 =  1;
              do
              {
                if (numArray1[index21] == index2 & numArray1[index21] > -1)
                {
                  numArray22[index21] = 1;
                  numArray1[index21] = -1;
                }
                index21 += 1;
              }
              while (index21 <= 6);
              let mut nr2: i32 =  this.game.Data.LandscapeTypeObj[index2].LayerSpriteID[this.game.SPRITE64[numArray22[1], numArray22[2], numArray22[3], numArray22[4], numArray22[5], numArray22[6]]];
              nr3: i32;
              index22: i32;
              if (this.game.Data.LandscapeTypeObj[index2].UseSheet)
              {
                nr3 = this.game.Data.LandscapeTypeObj[index2].SheetSpriteID;
                index22 = this.game.SPRITE64[numArray22[1], numArray22[2], numArray22[3], numArray22[4], numArray22[5], numArray22[6]];
              }
              else
              {
                nr3 = -1;
                index22 = -1;
              }
              if (nr2 > -1)
              {
                if (nr3 == -1)
                {
                  index2 = (int) Math.Round( BitmapStore.GetWidth(nr2) /  num6);
                  let mut num35: i32 =  0;
                  if (index2 > 1)
                    num35 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
                  ref Graphics local19 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr2, Zoom);
                  ref local20: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(num35 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local19, ref local20, srcrect, destrect);
                }
                else if ( this.game.Data.RuleVar[998] == 1.0 & this.game.Data.LandscapeTypeObj[index2].UsePreHexBorderTexture)
                {
                  let mut preHexTextureId: i32 =  this.game.Data.LandscapeTypeObj[index2].PreHexTextureID;
                  let mut sheetSpriteId: i32 =  this.game.Data.LandscapeTypeObj[index2].SheetSpriteID;
                  if (Zoom == 0)
                  {
                    BitmapData bitmapdata = this.tempHexMed.LockBits(Rectangle::new(0, 0, 64, 48), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
                    IntPtr scan0 = bitmapdata.Scan0;
                    let mut num36: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexMed.Height;
                    byte[] numArray23 = new byte[num36 - 1 + 1];
                    Marshal.Copy(scan0, numArray23, 0, num36);
                    let mut num37: i32 =  (cx + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (cy + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
                    Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheMed, num37 * num36, (Array) numArray23, 0, num36);
                    let mut index23: i32 =  0;
                    let mut num38: i32 =  numArray23.Length - 1;
                    for (let mut index24: i32 =  3; index24 <= num38; index24 += 4)
                    {
                      numArray23[index24] = BitmapStore.simpleByteCacheObj[sheetSpriteId].singleFredCacheMed[index22, index23];
                      index23 += 1;
                    }
                    Marshal.Copy(numArray23, 0, scan0, num36);
                    this.tempHexMed.UnlockBits(bitmapdata);
                    DrawMod.DrawSimple(ref toG, ref this.tempHexMed, tx, ty);
                  }
                  if (Zoom == 1)
                  {
                    BitmapData bitmapdata = this.tempHexBig.LockBits(Rectangle::new(0, 0, 128, 96), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
                    IntPtr scan0 = bitmapdata.Scan0;
                    let mut num39: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexBig.Height;
                    byte[] numArray24 = new byte[num39 - 1 + 1];
                    Marshal.Copy(scan0, numArray24, 0, num39);
                    let mut num40: i32 =  (cx + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal * 8 + (cy + BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal) % BitmapStore.simpleByteCacheObj[preHexTextureId].cacheTotal;
                    Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheBig, num40 * num39, (Array) numArray24, 0, num39);
                    let mut index25: i32 =  0;
                    let mut num41: i32 =  numArray24.Length - 1;
                    for (let mut index26: i32 =  3; index26 <= num41; index26 += 4)
                    {
                      numArray24[index26] = BitmapStore.simpleByteCacheObj[sheetSpriteId].singleFredCacheBig[index22, index25];
                      index25 += 1;
                    }
                    Marshal.Copy(numArray24, 0, scan0, num39);
                    this.tempHexBig.UnlockBits(bitmapdata);
                    DrawMod.DrawSimple(ref toG, ref this.tempHexBig, tx, ty);
                  }
                  if (Zoom == -1)
                  {
                    BitmapData bitmapdata = this.tempHexSmall.LockBits(Rectangle::new(0, 0, 32, 24), ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);
                    IntPtr scan0 = bitmapdata.Scan0;
                    let mut num42: i32 =  Math.Abs(bitmapdata.Stride) * this.tempHexSmall.Height;
                    byte[] numArray25 = new byte[num42 - 1 + 1];
                    Marshal.Copy(scan0, numArray25, 0, num42);
                    let mut num43: i32 =  0;
                    Buffer.BlockCopy((Array) BitmapStore.simpleByteCacheObj[preHexTextureId].cacheSmall, num43 * num42, (Array) numArray25, 0, num42);
                    let mut index27: i32 =  0;
                    let mut num44: i32 =  numArray25.Length - 1;
                    for (let mut index28: i32 =  3; index28 <= num44; index28 += 4)
                    {
                      numArray25[index28] = BitmapStore.simpleByteCacheObj[sheetSpriteId].singleFredCacheSmall[index22, index27];
                      index27 += 1;
                    }
                    Marshal.Copy(numArray25, 0, scan0, num42);
                    this.tempHexSmall.UnlockBits(bitmapdata);
                    DrawMod.DrawSimple(ref toG, ref this.tempHexSmall, tx, ty);
                  }
                }
                else
                {
                  ref Graphics local21 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr3, Zoom);
                  ref local22: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index22] * num6, this.game.SHEETY[index22] * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local21, ref local22, srcrect, destrect);
                }
              }
            }
          }
          if ( this.game.Data.RuleVar[998] == 1.0 & (this.game.Data.LandscapeTypeObj[lt1].UsePreHexTextureAndRegularPreHex | this.game.Data.LandscapeTypeObj[lt1].OverridesZ >= 899) && !this.game.Data.LandscapeTypeObj[lt1].Transparent)
          {
            index2 = (int) Math.Round( BitmapStore.GetBitmap(nr1, Zoom).Width /  num6);
            let mut num45: i32 =  0;
            if (index2 > 1)
              num45 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
            ref Graphics local23 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
            ref local24: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num45 * num6, 0, num6, num7);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, num6, num7);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local23, ref local24, srcrect, destrect);
          }
          int[] numArray26 = new int[7];
          int[] numArray27 = new int[7];
          int[] numArray28 = new int[7];
          let mut index29: i32 =  -1;
          let mut tfacing2: i32 =  1;
          do
          {
            coordinateArray[tfacing2] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing2);
            numArray26[tfacing2] = -1;
            numArray27[tfacing2] = -1;
            if (coordinateArray[tfacing2].onmap)
            {
              numArray26[tfacing2] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].LandscapeType;
              if (index5 > -1)
              {
                if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].get_SeeNow(index5) == 0)
                  numArray26[tfacing2] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].get_LastLT(index5);
                if (numArray26[tfacing2] == -1)
                  numArray26[tfacing2] = lt1;
              }
              numArray27[tfacing2] = numArray26[tfacing2];
              numArray28[tfacing2] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].SpriteNr;
              if (index5 > -1)
              {
                if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].get_SeeNow(index5) == 0)
                  numArray28[tfacing2] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].get_LastSpr(index5);
                if (numArray28[tfacing2] == -1)
                  numArray28[tfacing2] = index7;
              }
              if (this.game.Data.LandscapeTypeObj[numArray26[tfacing2]].Interior | this.game.Data.LandscapeTypeObj[numArray26[tfacing2]].ExtraExterior > -1)
              {
                if (!this.game.Data.LandscapeTypeObj[numArray26[tfacing2]].Transparent)
                  numArray26[tfacing2] = !this.game.Data.LandscapeTypeObj[lt1].ExtraExteriorSame ? (!(this.game.Data.LandscapeTypeObj[numArray26[tfacing2]].ExtraExterior > -1 & this.game.Data.LandscapeTypeObj[lt1].CheckOverride2(numArray26[tfacing2])) ? -1 : numArray26[tfacing2]) : (!this.game.Data.LandscapeTypeObj[lt1].CheckOverride2(numArray26[tfacing2]) ? -1 : numArray26[tfacing2]);
              }
              else
                numArray26[tfacing2] = -1;
            }
            if (coordinateArray[tfacing2].onmap && this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].SpriteNr <= this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].LandscapeType].BasicSpriteCounter && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].LandscapeType].OverIsTop[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing2].x, coordinateArray[tfacing2].y].SpriteNr] && tfacing2 != 4)
              numArray26[tfacing2] = -1;
            numArray3[tfacing2] = 0;
            tfacing2 += 1;
          }
          while (tfacing2 <= 6);
          let mut index30: i32 =  1;
          do
          {
            if (coordinateArray[index30].onmap & numArray26[index30] > -1 && this.game.Data.LandscapeTypeObj[numArray26[index30]].SpecialLayer | this.game.Data.LandscapeTypeObj[numArray26[index30]].ExtraExterior > -1 | this.game.Data.LandscapeTypeObj[numArray26[index30]].OverIsTop[numArray28[index30]] && numArray3[index30] == 0)
            {
              let mut extraExterior: i32 =  numArray26[index30];
              let mut index31: i32 =  1;
              do
              {
                if (numArray26[index31] == extraExterior)
                {
                  numArray4[index31] = 1;
                  numArray3[index31] = 1;
                }
                else
                  numArray4[index31] = numArray27[index31] <= -1 ? 0 : (!(this.game.Data.LandscapeTypeObj[numArray27[index31]].CheckOverride2(extraExterior) & numArray27[index31] == extraExterior) ? 0 : 1);
                index31 += 1;
              }
              while (index31 <= 6);
              index29 += 1;
              numArray10[index29] = -1;
              index2 = numArray28[index30];
              if (this.game.Data.LandscapeTypeObj[extraExterior].ExtraExterior > -1)
              {
                extraExterior = this.game.Data.LandscapeTypeObj[extraExterior].ExtraExterior;
                index2 = 0;
              }
              if (!this.game.Data.LandscapeTypeObj[extraExterior].OverIsTop[index2])
              {
                if (this.game.Data.LandscapeTypeObj[numArray26[index30]].SpecialLayer6)
                {
                  flagArray1[index29] = true;
                  if (numArray4[1] > 0)
                    numArray12[index29, 0] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[1, 0, 0, 0, 0, 0]];
                  if (numArray4[2] > 0)
                    numArray12[index29, 1] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[0, 1, 0, 0, 0, 0]];
                  if (numArray4[3] > 0)
                    numArray12[index29, 2] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[0, 0, 1, 0, 0, 0]];
                  if (numArray4[4] > 0)
                    numArray12[index29, 3] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 1, 0, 0]];
                  if (numArray4[5] > 0)
                    numArray12[index29, 4] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 0, 1, 0]];
                  if (numArray4[6] > 0)
                    numArray12[index29, 5] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 0, 0, 1]];
                }
                else if (!this.game.Data.LandscapeTypeObj[extraExterior].UseSheet)
                {
                  numArray10[index29] = -1;
                  numArray7[index29] = this.game.Data.LandscapeTypeObj[extraExterior].LayerSpriteID[this.game.SPRITE64[numArray4[1], numArray4[2], numArray4[3], numArray4[4], numArray4[5], numArray4[6]]];
                }
                else
                {
                  numArray10[index29] = this.game.Data.LandscapeTypeObj[extraExterior].SheetSpriteID;
                  numArray11[index29] = this.game.SPRITE64[numArray4[1], numArray4[2], numArray4[3], numArray4[4], numArray4[5], numArray4[6]];
                }
              }
              else
              {
                numArray7[index29] = this.game.Data.LandscapeTypeObj[extraExterior].BasicSpriteID3[index2];
                flagArray2[index29] = true;
                numArray10[index29] = -1;
              }
              numArray9[index29] = this.game.Data.LandscapeTypeObj[extraExterior].OverridesZ;
            }
            index30 += 1;
          }
          while (index30 <= 6);
          if (index29 > 0)
          {
            let mut num46: i32 =  index29;
            for (let mut index32: i32 =  0; index32 <= num46; index32 += 1)
            {
              let mut num47: i32 =  index29 - 1;
              for (let mut index33: i32 =  0; index33 <= num47; index33 += 1)
              {
                if (numArray9[index33] > numArray9[index33 + 1])
                {
                  index2 = numArray9[index33 + 1];
                  let mut num48: i32 =  numArray7[index33 + 1];
                  let mut num49: i32 =  numArray8[index33 + 1];
                  bool flag5 = flagArray1[index33 + 1];
                  let mut num50: i32 =  numArray10[index33 + 1];
                  let mut num51: i32 =  numArray11[index33 + 1];
                  bool flag6 = flagArray2[index33 + 1];
                  let mut index34: i32 =  0;
                  do
                  {
                    numArray4[index34] = numArray12[index33 + 1, index34];
                    index34 += 1;
                  }
                  while (index34 <= 5);
                  numArray9[index33 + 1] = numArray9[index33];
                  numArray7[index33 + 1] = numArray7[index33];
                  numArray8[index33 + 1] = numArray8[index33];
                  flagArray1[index33 + 1] = flagArray1[index33];
                  flagArray2[index33 + 1] = flagArray2[index33];
                  numArray10[index33 + 1] = numArray10[index33];
                  numArray11[index33 + 1] = numArray11[index33];
                  let mut index35: i32 =  0;
                  do
                  {
                    numArray12[index33 + 1, index35] = numArray12[index33, index35];
                    index35 += 1;
                  }
                  while (index35 <= 5);
                  numArray9[index33] = index2;
                  numArray7[index33] = num48;
                  numArray8[index33] = num49;
                  flagArray1[index33] = flag5;
                  flagArray2[index33] = flag6;
                  numArray10[index33] = num50;
                  numArray11[index33] = num51;
                  let mut index36: i32 =  0;
                  do
                  {
                    numArray12[index33, index36] = numArray4[index36];
                    index36 += 1;
                  }
                  while (index36 <= 5);
                }
              }
            }
          }
          if (index29 > -1)
          {
            let mut num52: i32 =  index29;
            for (let mut index37: i32 =  0; index37 <= num52; index37 += 1)
            {
              if (flagArray1[index37])
              {
                let mut index38: i32 =  0;
                do
                {
                  if (numArray12[index37, index38] > 0 && numArray9[index37] <= 900)
                  {
                    index2 = (int) Math.Round( BitmapStore.GetWidth(numArray12[index37, index38], Zoom) /  num6);
                    let mut num53: i32 =  0;
                    if (index2 > 1)
                    {
                      Random random = new Random((int) Math.Round(a1));
                      if (flagArray2[index37])
                        random = new Random((int) Math.Round(a2));
                      num53 = random.Next(0, index2 - 1);
                    }
                    ref Graphics local25 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(numArray12[index37, index38], Zoom);
                    ref local26: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num53 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local25, ref local26, srcrect, destrect);
                  }
                  index38 += 1;
                }
                while (index38 <= 5);
              }
              else if (numArray9[index37] <= 900)
              {
                if (numArray10[index37] > -1)
                {
                  ref Graphics local27 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(numArray10[index37], Zoom);
                  ref local28: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(this.game.SHEETX[numArray11[index37]] * num6, this.game.SHEETY[numArray11[index37]] * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local27, ref local28, srcrect, destrect);
                }
                else
                {
                  index2 = (int) Math.Round( BitmapStore.GetWidth(numArray7[index37], Zoom) /  num6);
                  let mut num54: i32 =  0;
                  if (index2 > 1)
                  {
                    Random random = new Random((int) Math.Round(a1));
                    if (flagArray2[index37])
                      random = new Random((int) Math.Round(a2));
                    num54 = random.Next(0, index2 - 1);
                  }
                  ref Graphics local29 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(numArray7[index37], Zoom);
                  ref local30: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(num54 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local29, ref local30, srcrect, destrect);
                }
              }
            }
          }
          let mut lt2: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
          let mut index39: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].SpriteNr;
          if (index39 == 2)
            cx = cx;
          if (index5 > -1)
          {
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              lt2 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              index39 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastSpr(index5);
          }
          int[] numArray29 = new int[7];
          if (lt2 > -1)
          {
            if (this.game.Data.LandscapeTypeObj[lt2].OverridesZ != 899 && !this.game.Data.LandscapeTypeObj[lt2].Transparent)
            {
              nr1 = lt2 <= -1 ? this.game.NOHEX : (!(index39 > -1 & index39 <= this.game.Data.LandscapeTypeObj[lt2].BasicSpriteCounter) ? this.game.NOHEX : this.game.Data.LandscapeTypeObj[lt2].BasicSpriteID[index39]);
              let mut nr4: i32 =  -1;
              let mut index40: i32 =  -1;
              if (this.game.Data.LandscapeTypeObj[lt2].Interior)
              {
                let mut tfacing3: i32 =  1;
                do
                {
                  coordinateArray[tfacing3] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing3);
                  numArray29[tfacing3] = 0;
                  if (coordinateArray[tfacing3].onmap)
                  {
                    let mut num55: i32 =  this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing3].x, coordinateArray[tfacing3].y].LandscapeType;
                    if (index5 > -1)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing3].x, coordinateArray[tfacing3].y].get_SeeNow(index5) == 0)
                        num55 = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing3].x, coordinateArray[tfacing3].y].get_LastLT(index5);
                      if (num55 == -1)
                        num55 = lt2;
                    }
                    if (num55 == lt2)
                    {
                      numArray29[tfacing3] = 1;
                    }
                    else
                    {
                      let mut overridesCount: i32 =  this.game.Data.LandscapeTypeObj[lt2].OverridesCount;
                      for (let mut index41: i32 =  0; index41 <= overridesCount; index41 += 1)
                      {
                        if (this.game.Data.LandscapeTypeObj[lt2].OverridesType[index41] == num55)
                        {
                          numArray29[tfacing3] = 1;
                          if (this.game.Data.LandscapeTypeObj[lt2].ExtraExterior > -1 & !this.game.Data.LandscapeTypeObj[lt2].ExtraExteriorSame)
                            numArray29[tfacing3] = 0;
                        }
                      }
                    }
                  }
                  else
                    numArray29[tfacing3] = 1;
                  tfacing3 += 1;
                }
                while (tfacing3 <= 6);
                nr1 = this.game.Data.LandscapeTypeObj[lt2].LayerSpriteID[this.game.SPRITE64[numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5], numArray29[6]]];
                if (this.game.Data.LandscapeTypeObj[lt2].UseSheet)
                {
                  nr4 = this.game.Data.LandscapeTypeObj[lt2].SheetSpriteID;
                  index40 = this.game.SPRITE64[numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5], numArray29[6]];
                }
                else
                {
                  nr4 = -1;
                  index40 = -1;
                }
              }
              if (flag2 & this.game.Data.LandscapeTypeObj[lt2].IsSea)
              {
                if (!BitmapStore.IsKnownTransBitmap(nr1))
                {
                  if (nr4 == -1)
                  {
                    index2 = (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
                    let mut num56: i32 =  0;
                    if (index2 > 1)
                      num56 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
                    ref Graphics local31 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
                    ref local32: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num56 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local31, ref local32, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local33 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr4, Zoom);
                    ref local34: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index40] * num6, this.game.SHEETY[index40] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local33, ref local34, srcrect, destrect);
                  }
                }
                ref Graphics local35 = ref toG;
                bitmap1 = BitmapStore.GetBitmap(this.game.ROUNDBALL, Zoom);
                ref local36: Bitmap = ref bitmap1;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local35, ref local36, x, y);
              }
              else if (!BitmapStore.IsKnownTransBitmap(nr1))
              {
                if (nr4 == -1)
                {
                  index2 = (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
                  let mut num57: i32 =  0;
                  if (index2 > 1)
                    num57 = new Random((int) Math.Round(a1)).Next(0, index2 - 1);
                  ref Graphics local37 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local38: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(num57 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local37, ref local38, srcrect, destrect);
                }
                else
                {
                  ref Graphics local39 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr4, Zoom);
                  ref local40: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index40] * num6, this.game.SHEETY[index40] * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local39, ref local40, srcrect, destrect);
                }
              }
            }
          }
          else
          {
            nr1 = this.game.NOHEX;
            ref Graphics local41 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
            ref local42: Bitmap = ref bitmap1;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local41, ref local42, x, y);
          }
          let mut index42: i32 =  -1;
          let mut tfacing4: i32 =  1;
          do
          {
            coordinateArray[tfacing4] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing4);
            numArray26[tfacing4] = -1;
            numArray2[tfacing4] = 0;
            if (coordinateArray[tfacing4].onmap)
            {
              numArray26[tfacing4] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].LandscapeType;
              if (index5 > -1)
              {
                if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].get_SeeNow(index5) == 0)
                  numArray26[tfacing4] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].get_LastLT(index5);
                if (numArray26[tfacing4] == -1)
                  numArray26[tfacing4] = lt2;
              }
              numArray2[tfacing4] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].SpriteNr;
              if (index5 > -1)
              {
                if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].get_SeeNow(index5) == 0)
                  numArray28[tfacing4] = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].get_LastSpr(index5);
                if (numArray28[tfacing4] == -1)
                  numArray28[tfacing4] = index39;
              }
              if (this.game.Data.LandscapeTypeObj[numArray26[tfacing4]].Transparent)
                numArray26[tfacing4] = -1;
              else if (this.game.Data.LandscapeTypeObj[numArray26[tfacing4]].Interior)
                numArray26[tfacing4] = -1;
              else if (this.game.Data.LandscapeTypeObj[numArray26[tfacing4]].UsePreHexTexture & !this.game.Data.LandscapeTypeObj[numArray26[tfacing4]].UsePreHexTextureAndRegularPreHex)
                numArray26[tfacing4] = -1;
              else if (!this.game.Data.LandscapeTypeObj[numArray26[tfacing4]].CheckOverride(lt2))
                numArray26[tfacing4] = -1;
            }
            if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].SpriteNr <= this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].LandscapeType].BasicSpriteCounter && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].LandscapeType].OverIsTop[this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing4].x, coordinateArray[tfacing4].y].SpriteNr] && tfacing4 != 4)
              numArray26[tfacing4] = -1;
            numArray3[tfacing4] = 0;
            tfacing4 += 1;
          }
          while (tfacing4 <= 6);
          let mut index43: i32 =  1;
          do
          {
            let mut index44: i32 =  1;
            do
            {
              if (numArray26[index43] > -1 & numArray26[index44] > -1 & numArray26[index43] != numArray26[index44] && this.game.Data.LandscapeTypeObj[numArray26[index43]].CheckOverride2(numArray26[index44]) && this.game.Data.LandscapeTypeObj[numArray26[index44]].CheckOverride2(numArray26[index43]))
              {
                if (numArray26[index43] > numArray26[index44])
                  numArray26[index44] = numArray26[index43];
                else
                  numArray26[index43] = numArray26[index44];
              }
              index44 += 1;
            }
            while (index44 <= 6);
            index43 += 1;
          }
          while (index43 <= 6);
          if (cx == 4 & cy == 7)
            cx = cx;
          let mut index45: i32 =  1;
          do
          {
            if (coordinateArray[index45].onmap & numArray26[index45] > -1 && numArray2[index45] <= this.game.Data.LandscapeTypeObj[numArray26[index45]].BasicSpriteCounter && this.game.Data.LandscapeTypeObj[numArray26[index45]].SpecialLayer | this.game.Data.LandscapeTypeObj[numArray26[index45]].OverIsTop[numArray2[index45]] && numArray3[index45] == 0)
            {
              let mut index46: i32 =  numArray26[index45];
              let mut index47: i32 =  1;
              do
              {
                if (numArray26[index47] == index46)
                {
                  numArray29[index47] = 1;
                  numArray3[index47] = 1;
                }
                else
                  numArray29[index47] = 0;
                index47 += 1;
              }
              while (index47 <= 6);
              index42 += 1;
              numArray10[index42] = -1;
              if (!this.game.Data.LandscapeTypeObj[index46].OverIsTop[numArray2[index45]])
              {
                if (this.game.Data.LandscapeTypeObj[index46].SpecialLayer6)
                {
                  flagArray1[index42] = true;
                  numArray10[index42] = -1;
                  if (numArray29[1] > 0)
                    numArray12[index42, 0] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[1, 0, 0, 0, 0, 0]];
                  if (numArray29[2] > 0)
                    numArray12[index42, 1] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[0, 1, 0, 0, 0, 0]];
                  if (numArray29[3] > 0)
                    numArray12[index42, 2] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[0, 0, 1, 0, 0, 0]];
                  if (numArray29[4] > 0)
                    numArray12[index42, 3] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 1, 0, 0]];
                  if (numArray29[5] > 0)
                    numArray12[index42, 4] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 0, 1, 0]];
                  if (numArray29[6] > 0)
                    numArray12[index42, 5] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[0, 0, 0, 0, 0, 1]];
                }
                else if (this.game.Data.LandscapeTypeObj[index46].UseSheet)
                {
                  numArray10[index42] = this.game.Data.LandscapeTypeObj[index46].SheetSpriteID;
                  numArray11[index42] = this.game.SPRITE64[numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5], numArray29[6]];
                }
                else
                {
                  numArray10[index42] = -1;
                  numArray7[index42] = this.game.Data.LandscapeTypeObj[index46].LayerSpriteID[this.game.SPRITE64[numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5], numArray29[6]]];
                }
              }
              else
              {
                numArray10[index42] = -1;
                numArray7[index42] = this.game.Data.LandscapeTypeObj[index46].BasicSpriteID3[numArray2[index45]];
                flagArray2[index42] = true;
              }
              numArray9[index42] = this.game.Data.LandscapeTypeObj[index46].OverridesZ;
              numArray8[index42] = !this.game.Data.LandscapeTypeObj[numArray26[index45]].IsSea ? 0 : 1;
            }
            index45 += 1;
          }
          while (index45 <= 6);
          if (index42 > 0)
          {
            let mut num58: i32 =  index42;
            for (let mut index48: i32 =  0; index48 <= num58; index48 += 1)
            {
              let mut num59: i32 =  index42 - 1;
              for (let mut index49: i32 =  0; index49 <= num59; index49 += 1)
              {
                if (numArray9[index49] > numArray9[index49 + 1])
                {
                  index2 = numArray9[index49 + 1];
                  let mut num60: i32 =  numArray7[index49 + 1];
                  let mut num61: i32 =  numArray8[index49 + 1];
                  bool flag7 = flagArray1[index49 + 1];
                  bool flag8 = flagArray2[index49 + 1];
                  let mut num62: i32 =  numArray10[index49 + 1];
                  let mut num63: i32 =  numArray11[index49 + 1];
                  let mut index50: i32 =  0;
                  do
                  {
                    numArray29[index50] = numArray12[index49 + 1, index50];
                    index50 += 1;
                  }
                  while (index50 <= 5);
                  numArray9[index49 + 1] = numArray9[index49];
                  numArray7[index49 + 1] = numArray7[index49];
                  numArray8[index49 + 1] = numArray8[index49];
                  flagArray1[index49 + 1] = flagArray1[index49];
                  flagArray2[index49 + 1] = flagArray2[index49];
                  numArray10[index49 + 1] = numArray10[index49];
                  numArray11[index49 + 1] = numArray11[index49];
                  let mut index51: i32 =  0;
                  do
                  {
                    numArray12[index49 + 1, index51] = numArray12[index49, index51];
                    index51 += 1;
                  }
                  while (index51 <= 5);
                  numArray9[index49] = index2;
                  numArray7[index49] = num60;
                  numArray8[index49] = num61;
                  flagArray1[index49] = flag7;
                  flagArray2[index49] = flag8;
                  numArray10[index49] = num62;
                  numArray11[index49] = num63;
                  let mut index52: i32 =  0;
                  do
                  {
                    numArray12[index49, index52] = numArray29[index52];
                    index52 += 1;
                  }
                  while (index52 <= 5);
                }
              }
            }
          }
          if (index42 > -1)
          {
            let mut num64: i32 =  index42;
            for (let mut index53: i32 =  0; index53 <= num64; index53 += 1)
            {
              if (numArray8[index53] == 0 & numArray9[index53] <= 898)
              {
                if (flagArray1[index53])
                {
                  let mut index54: i32 =  0;
                  do
                  {
                    if (numArray12[index53, index54] > 0 && numArray9[index53] <= 900)
                    {
                      index2 = (int) Math.Round( BitmapStore.GetWidth(numArray12[index53, index54], Zoom) /  num6);
                      let mut num65: i32 =  0;
                      if (index2 > 1)
                      {
                        Random random = new Random((int) Math.Round(a1));
                        if (flagArray2[index53])
                          random = new Random((int) Math.Round(a2));
                        num65 = random.Next(0, index2 - 1);
                      }
                      ref Graphics local43 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(numArray12[index53, index54], Zoom);
                      ref local44: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num65 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local43, ref local44, srcrect, destrect);
                    }
                    index54 += 1;
                  }
                  while (index54 <= 5);
                }
                else if (numArray9[index53] <= 900)
                {
                  if (numArray10[index53] == -1)
                  {
                    index2 = (int) Math.Round( BitmapStore.GetWidth(numArray7[index53], Zoom) /  num6);
                    let mut num66: i32 =  0;
                    if (index2 > 1)
                    {
                      Random random = new Random((int) Math.Round(a1));
                      if (flagArray2[index53])
                        random = new Random((int) Math.Round(a2));
                      num66 = random.Next(0, index2 - 1);
                    }
                    ref Graphics local45 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(numArray7[index53], Zoom);
                    ref local46: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num66 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local45, ref local46, srcrect, destrect);
                  }
                  else
                  {
                    ref Graphics local47 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(numArray10[index53], Zoom);
                    ref local48: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[numArray11[index53]] * num6, this.game.SHEETY[numArray11[index53]] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local47, ref local48, srcrect, destrect);
                  }
                }
              }
            }
          }
          let mut index55: i32 =  -1;
          let mut num67: i32 =  -1;
          if ((this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.ShrowdOn) & !flag1)
            num67 = this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime;
          else if (index5 > -1)
            num67 = this.game.Data.MapObj[0].HexObj[cx, cy].get_LastReg(index5);
          if (this.game.EditObj.RealRound > 0)
          {
            if (UseRegimeColoring & num67 > -1 & num67 != index5 | flag1)
            {
              index55 = num67;
              if (flag1)
                index55 = this.game.EditObj.HisOwner[cmap].Value[cx, cy];
              if (flag1 && index55 == index5)
                index55 = -1;
              if (index55 == -2)
                index55 = this.game.Data.MapObj[0].HexObj[cx, cy].get_LastReg(index5);
            }
            else if (this.game.Data.LandscapeTypeObj[lt2].Interior & UseRegimeColoring & this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == -1 & (index5 == -1 | num67 != index5))
            {
              int[] numArray30 = new int[7];
              let mut num68: i32 =  0;
              let mut tfacing5: i32 =  1;
              do
              {
                numArray30[tfacing5] = -1;
                coordinateArray[tfacing5] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing5);
                if (coordinateArray[tfacing5].onmap && this.game.Data.MapObj[0].HexObj[coordinateArray[tfacing5].x, coordinateArray[tfacing5].y].Regime > -1)
                {
                  numArray30[tfacing5] = this.game.Data.MapObj[0].HexObj[coordinateArray[tfacing5].x, coordinateArray[tfacing5].y].Regime;
                  num68 += 1;
                }
                tfacing5 += 1;
              }
              while (tfacing5 <= 6);
              if (num68 > 0)
              {
                let mut index56: i32 =  1;
                num69: i32;
                num70: i32;
                do
                {
                  let mut num71: i32 =  0;
                  if (numArray30[index56] > -1)
                  {
                    let mut index57: i32 =  1;
                    do
                    {
                      if (numArray30[index56] == numArray30[index57])
                        num71 += 1;
                      index57 += 1;
                    }
                    while (index57 <= 6);
                  }
                  if (num71 > num69)
                  {
                    num69 = num71;
                    num70 = numArray30[index56];
                  }
                  index56 += 1;
                }
                while (index56 <= 6);
                if (num69 > 0)
                  index55 = num70;
              }
            }
            if (index55 > -1 & index55 <= this.game.Data.RegimeCounter)
            {
              if (index5 > -1 && index55 > -1 && this.game.Data.RegimeObj[index5].RegimeRel[index55] == 0)
              {
                index2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.strId143slot].GetData(0, this.game.Data.RegimeObj[index55].id, 1)));
                if (index2 == 2)
                {
                  let mut strId275slot: i32 =  this.strId275slot;
                  if (this.cacheDipClear[this.game.Data.RegimeObj[index5].id, this.game.Data.RegimeObj[index55].id] == 0)
                    index55 = 1;
                }
              }
              if (index55 > -1 & index55 != index5 && this.game.Data.LandscapeTypeObj[lt2].OverridesZ < 899)
              {
                if (Information.IsNothing( this.game.Data.RegimeObj[index55].TempRegimeColor))
                  this.game.Data.RegimeObj[index55].doTempRegimeHighlight();
                if (Zoom == -1)
                  DrawMod.DrawSimple(ref toG, ref this.game.Data.RegimeObj[index55].TempRegimeColorSmall, tx, ty);
                if (Zoom == 0)
                  DrawMod.DrawSimple(ref toG, ref this.game.Data.RegimeObj[index55].TempRegimeColor, tx, ty);
                if (Zoom == 1)
                  DrawMod.DrawSimple(ref toG, ref this.game.Data.RegimeObj[index55].TempRegimeColorBig, tx, ty);
              }
            }
          }
          let mut landscapeType1: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
          if (!this.game.EditObj.skipGfxDetail && this.game.AllowHeightMap & this.game.Data.LandscapeTypeObj[landscapeType1].IsSea & this.game.Data.LandscapeTypeObj[landscapeType1].Interior)
            this.DrawHeightMap(ref toG, cx, cy, cmap, tx, ty, Zoom, true, ref gBitmap);
          if (this.game.Data.LandscapeTypeObj[landscapeType1].IsSea & this.game.Data.LandscapeTypeObj[landscapeType1].Interior & this.game.Data.LandscapeTypeObj[landscapeType1].OverridesZ == 899)
          {
            let mut num72: i32 =  0;
            do
            {
              num73: i32;
              if (num72 == 0)
              {
                index2 = 2;
                index19 = (int) Math.Round( -num6 / 6.0);
                index55 = (int) Math.Round(-( num7 / 2.0) - 1.0);
                num5 = 5;
                index8 = (int) Math.Round( (num6 * 2) / 3.0 - 1.0);
                num73 = -1;
              }
              if (num72 == 1)
              {
                index2 = 3;
                index19 = (int) Math.Round( num6 / 6.0 + 1.0);
                index55 = (int) Math.Round(-( num7 / 2.0));
                num5 = 0;
                index8 = (int) Math.Round( num6 / 6.0 + 1.0);
                num73 = (int) Math.Round( num7 / 2.0);
              }
              if (num72 == 2)
              {
                index2 = 4;
                index19 = (int) Math.Round( (num6 * 2) / 3.0);
                index55 = 0;
                num5 = 1;
                index8 = (int) Math.Round(-( num6 / 6.0) + 1.0);
                num73 = (int) Math.Round( num7 / 2.0);
              }
              if (num72 == 3)
              {
                index2 = 5;
                index19 = (int) Math.Round( num6 / 6.0);
                index55 = (int) Math.Round( num7 / 2.0);
                num5 = 2;
                index8 = (int) Math.Round(-( (num6 * 2) / 3.0) + 1.0);
                num73 = 0;
              }
              if (num72 == 4)
              {
                index2 = 0;
                index19 = (int) Math.Round(-( num6 / 6.0) - 1.0);
                index55 = (int) Math.Round( num7 / 2.0);
                num5 = 3;
                index8 = (int) Math.Round(-( num6 / 6.0) - 1.0);
                num73 = (int) Math.Round(-( num7 / 2.0));
              }
              if (num72 == 5)
              {
                index2 = 1;
                index19 = (int) Math.Round(-( (num6 * 2) / 3.0));
                index55 = 0;
                num5 = 4;
                index8 = (int) Math.Round( num6 / 6.0 - 1.0);
                num73 = (int) Math.Round(-( num7 / 2.0));
              }
              coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, num72 + 1);
              if (coordinate1.onmap)
              {
                let mut index58: i32 =  this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RiverType[index2];
                if (index58 > -1 && !this.game.Data.RiverTypeObj[index58].snakeMode)
                {
                  let mut index59: i32 =  0;
                  do
                  {
                    numArray29[index59] = 0;
                    if (index59 == index2)
                      numArray29[index59] = 1;
                    index59 += 1;
                  }
                  while (index59 <= 5);
                  num34 = this.game.Data.RiverTypeObj[index58].LayerSpriteID[this.game.SPRITE64[numArray29[0], numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5]]];
                  let mut sheetSpriteId1: i32 =  this.game.Data.RiverTypeObj[index58].SheetSpriteID;
                  if (sheetSpriteId1 > 0)
                  {
                    let mut index60: i32 =  this.game.SPRITE64[numArray29[0], numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5]];
                    ref Graphics local49 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(sheetSpriteId1, Zoom);
                    ref local50: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index60] * num6, this.game.SHEETY[index60] * num7, num6, num7);
                    let mut srcrect3: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx + index19, ty + index55, num6, num7);
                    let mut destrect3: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local49, ref local50, srcrect3, destrect3);
                    let mut index61: i32 =  0;
                    do
                    {
                      numArray29[index61] = 0;
                      if (index61 == num5)
                        numArray29[index61] = 1;
                      index61 += 1;
                    }
                    while (index61 <= 5);
                    num34 = this.game.Data.RiverTypeObj[index58].LayerSpriteID[this.game.SPRITE64[numArray29[0], numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5]]];
                    let mut sheetSpriteId2: i32 =  this.game.Data.RiverTypeObj[index58].SheetSpriteID;
                    if (sheetSpriteId2 > 0)
                    {
                      let mut index62: i32 =  this.game.SPRITE64[numArray29[0], numArray29[1], numArray29[2], numArray29[3], numArray29[4], numArray29[5]];
                      ref Graphics local51 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(sheetSpriteId2, Zoom);
                      ref local52: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index62] * num6, this.game.SHEETY[index62] * num7, num6, num7);
                      let mut srcrect4: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx + index8, ty + num73, num6, num7);
                      let mut destrect4: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local51, ref local52, srcrect4, destrect4);
                    }
                  }
                }
              }
              num72 += 1;
            }
            while (num72 <= 5);
          }
          let mut index63: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
          let mut index64: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].SpriteNr;
          if (index5 > -1)
          {
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              index63 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              index64 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastSpr(index5);
          }
          int[] numArray31 = new int[7];
          if (index63 > -1 && this.game.Data.LandscapeTypeObj[index63].OverridesZ == 899 && !this.game.Data.LandscapeTypeObj[index63].Transparent)
          {
            nr1 = index63 <= -1 ? this.game.NOHEX : (index64 <= -1 ? this.game.NOHEX : this.game.Data.LandscapeTypeObj[index63].BasicSpriteID[index64]);
            let mut nr5: i32 =  -1;
            let mut index65: i32 =  -1;
            if (this.game.Data.LandscapeTypeObj[index63].Interior)
            {
              let mut tfacing6: i32 =  1;
              do
              {
                coordinateArray[tfacing6] = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing6);
                numArray31[tfacing6] = 0;
                if (coordinateArray[tfacing6].onmap)
                {
                  let mut num74: i32 =  this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing6].x, coordinateArray[tfacing6].y].LandscapeType;
                  if (index5 > -1)
                  {
                    if (this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing6].x, coordinateArray[tfacing6].y].get_SeeNow(index5) == 0)
                      num74 = this.game.Data.MapObj[cmap].HexObj[coordinateArray[tfacing6].x, coordinateArray[tfacing6].y].get_LastLT(index5);
                    if (num74 == -1)
                      num74 = index63;
                  }
                  if (num74 == index63)
                  {
                    numArray31[tfacing6] = 1;
                  }
                  else
                  {
                    let mut overridesCount: i32 =  this.game.Data.LandscapeTypeObj[index63].OverridesCount;
                    for (let mut index66: i32 =  0; index66 <= overridesCount; index66 += 1)
                    {
                      if (this.game.Data.LandscapeTypeObj[index63].OverridesType[index66] == num74)
                      {
                        numArray31[tfacing6] = 1;
                        if (this.game.Data.LandscapeTypeObj[index63].ExtraExterior > -1 & !this.game.Data.LandscapeTypeObj[index63].ExtraExteriorSame)
                          numArray31[tfacing6] = 0;
                      }
                    }
                  }
                }
                else
                  numArray31[tfacing6] = 1;
                tfacing6 += 1;
              }
              while (tfacing6 <= 6);
              nr1 = this.game.Data.LandscapeTypeObj[index63].LayerSpriteID[this.game.SPRITE64[numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5], numArray31[6]]];
              if (this.game.Data.LandscapeTypeObj[index63].UseSheet)
              {
                nr5 = this.game.Data.LandscapeTypeObj[index63].SheetSpriteID;
                index65 = this.game.SPRITE64[numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5], numArray31[6]];
              }
              else
              {
                nr5 = -1;
                index65 = -1;
              }
            }
            if (flag2 & this.game.Data.LandscapeTypeObj[index63].IsSea)
            {
              if (!BitmapStore.IsKnownTransBitmap(nr1))
              {
                let mut num75: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
                let mut num76: i32 =  0;
                if (num75 > 1)
                  num76 = new Random((int) Math.Round(a1)).Next(0, num75 - 1);
                if (nr5 == -1)
                {
                  ref Graphics local53 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local54: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(num76 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local53, ref local54, srcrect, destrect);
                }
                else
                {
                  ref Graphics local55 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr5, Zoom);
                  ref local56: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(this.game.SHEETX[index65] * num6, this.game.SHEETY[index65] * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local55, ref local56, srcrect, destrect);
                }
              }
              ref Graphics local57 = ref toG;
              bitmap1 = BitmapStore.GetBitmap(this.game.ROUNDBALL, Zoom);
              ref local58: Bitmap = ref bitmap1;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local57, ref local58, x, y);
            }
            else if (!BitmapStore.IsKnownTransBitmap(nr1))
            {
              let mut num77: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
              let mut num78: i32 =  0;
              if (num77 > 1)
                num78 = new Random((int) Math.Round(a1)).Next(0, num77 - 1);
              if (nr5 == -1)
              {
                ref Graphics local59 = ref toG;
                bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local60: Bitmap = ref bitmap1;
                rectangle2 = Rectangle::new(num78 * num6, 0, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local59, ref local60, srcrect, destrect);
              }
              else
              {
                ref Graphics local61 = ref toG;
                bitmap1 = BitmapStore.GetBitmap(nr5, Zoom);
                ref local62: Bitmap = ref bitmap1;
                rectangle2 = Rectangle::new(this.game.SHEETX[index65] * num6, this.game.SHEETY[index65] * num7, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local61, ref local62, srcrect, destrect);
              }
            }
          }
          if (num15 == 0 & index63 > -1)
          {
            if ((flag4 | flag2) & !this.game.Data.LandscapeTypeObj[index63].IsSea)
            {
              if (this.game.EditObj.RegimeColoring)
              {
                if (flag4)
                {
                  ref Graphics local63 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS3, Zoom);
                  ref local64: Bitmap = ref bitmap1;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.DrawSimple(ref local63, ref local64, x, y);
                }
                else if (this.game.Data.MapObj[0].HexObj[cx, cy].Regime == index5 | this.game.Data.Round == 0)
                {
                  ref Graphics local65 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS, Zoom);
                  ref local66: Bitmap = ref bitmap1;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.DrawSimple(ref local65, ref local66, x, y);
                }
                else
                {
                  ref Graphics local67 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS2, Zoom);
                  ref local68: Bitmap = ref bitmap1;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.DrawSimple(ref local67, ref local68, x, y);
                }
              }
              else if (flag4)
              {
                ref Graphics local69 = ref toG;
                bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS3, Zoom);
                ref local70: Bitmap = ref bitmap1;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local69, ref local70, x, y);
              }
              else
              {
                ref Graphics local71 = ref toG;
                bitmap1 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS, Zoom);
                ref local72: Bitmap = ref bitmap1;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local71, ref local72, x, y);
              }
              num15 = 1;
            }
            else if (cx == 13 & cy == 4)
              cx = cx;
          }
          int[] numArray32 = new int[7];
          if ( this.game.Data.RuleVar[32] == -1.0)
          {
            if ( this.game.Data.RuleVar[908] < 1.0)
            {
              bool flag9 = false;
              let mut index67: i32 =  0;
              do
              {
                let mut index68: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index67];
                if (index68 > -1 & numArray32[index67] == 0)
                {
                  if (index68 == 4 & !flag9)
                  {
                    flag9 = true;
                    this.DrawCanyon(ref toG, cx, cy, cmap, tx, ty, Zoom, ref gBitmap);
                  }
                  if (!this.game.Data.RiverTypeObj[index68].Transparent && this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index67] > -1 & !this.game.Data.RiverTypeObj[index68].SpecialLayer | this.game.Data.RiverTypeObj[index68].SpecialLayer)
                  {
                    nr6: i32;
                    if (!this.game.Data.RiverTypeObj[index68].SpecialLayer)
                    {
                      nr6 = this.game.Data.RiverTypeObj[index68].BasicSpriteID[index67];
                    }
                    else
                    {
                      let mut index69: i32 =  0;
                      do
                      {
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index69] == this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index67])
                        {
                          numArray31[index69] = 1;
                          numArray32[index69] = 1;
                        }
                        else
                          numArray31[index69] = 0;
                        index69 += 1;
                      }
                      while (index69 <= 5);
                      nr6 = this.game.Data.RiverTypeObj[index68].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                    }
                    if (this.game.Data.RiverTypeObj[index68].UseSheet)
                    {
                      let mut sheetSpriteId: i32 =  this.game.Data.RiverTypeObj[index68].SheetSpriteID;
                      let mut index70: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                      ref Graphics local73 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                      ref local74: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(this.game.SHEETX[index70] * num6, this.game.SHEETY[index70] * num7, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local73, ref local74, srcrect, destrect);
                    }
                    else
                    {
                      let mut num79: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr6, Zoom) /  num6);
                      let mut num80: i32 =  0;
                      if (!this.game.Data.RiverTypeObj[index68].snakeMode)
                      {
                        coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index67 + 1);
                        if (num79 > 1)
                          num80 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num79 - 1) : new Random((int) Math.Round(a1)).Next(0, num79 - 1);
                      }
                      else
                      {
                        index55 = index67 - 1;
                        let mut index71: i32 =  index67 + 1;
                        if (index55 < 0)
                          index55 = 5;
                        if (index71 > 5)
                          index71 = 0;
                        index8 = this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index55];
                        let mut num81: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index71];
                        if (index8 == index68 & num81 == index68)
                          num80 = 0;
                        if (index8 == index68 & num81 != index68)
                          num80 = 1;
                        if (index8 != index68 & num81 == index68)
                          num80 = 2;
                        if (index8 != index68 & num81 != index68)
                          num80 = 3;
                      }
                      ref Graphics local75 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(nr6, Zoom);
                      ref local76: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num80 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local75, ref local76, srcrect, destrect);
                    }
                  }
                }
                index67 += 1;
              }
              while (index67 <= 5);
            }
            numArray32 = new int[7];
            numArray31 = new int[7];
            bool flag10 = false;
            if (index5 > -1)
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0)
                flag10 = true;
            }
            else
              flag10 = true;
            if (!this.game.Data.FOWOn)
              flag10 = true;
            if (InfoMode)
              flag10 = false;
            if (flag10)
            {
              bool flag11 = false;
              let mut index72: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index72] > -1 & numArray32[index72] == 0)
                {
                  let mut index73: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index72];
                  if (!this.game.Data.RoadTypeObj[index73].Transparent && !this.game.Data.RoadTypeObj[index73].SpecialLayer && this.game.Data.RoadTypeObj[index73].useCenter6 & !flag11)
                  {
                    flag11 = true;
                    let mut center6spriteId: i32 =  this.game.Data.RoadTypeObj[index73].center6spriteId;
                    ref Graphics local77 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(center6spriteId, Zoom);
                    ref local78: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(0, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local77, ref local78, srcrect, destrect);
                  }
                }
                index72 += 1;
              }
              while (index72 <= 5);
              let mut index74: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index74] > -1)
                {
                  let mut index75: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index74];
                  if (!this.game.Data.RoadTypeObj[index75].Transparent)
                  {
                    if (!this.game.Data.RoadTypeObj[index75].SpecialLayer)
                    {
                      let mut nr7: i32 =  this.game.Data.RoadTypeObj[index75].BasicSpriteID[index74];
                      let mut num82: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr7, Zoom) /  num6);
                      let mut num83: i32 =  0;
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index74 + 1);
                      if (num82 > 1)
                        num83 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num82 - 1) : new Random((int) Math.Round(a1)).Next(0, num82 - 1);
                      ref Graphics local79 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(nr7, Zoom);
                      ref local80: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num83 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local79, ref local80, srcrect, destrect);
                    }
                    else
                    {
                      if (this.game.Data.RoadTypeObj[index75].FirstDrawOther > -1)
                      {
                        let mut index76: i32 =  0;
                        do
                        {
                          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index76] == this.game.Data.RoadTypeObj[index75].FirstDrawOther)
                          {
                            numArray31[index76] = 1;
                            numArray32[index76] = 1;
                          }
                          else
                            numArray31[index76] = this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index76] <= -1 ? 0 : (!(this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index76]].Category == this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index75].FirstDrawOther].Category & this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index75].FirstDrawOther].Category > -1) ? 0 : 1);
                          index76 += 1;
                        }
                        while (index76 <= 5);
                        if (this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index75].FirstDrawOther].UseSheet)
                        {
                          let mut sheetSpriteId: i32 =  this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index75].FirstDrawOther].SheetSpriteID;
                          let mut index77: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                          ref Graphics local81 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                          ref local82: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(this.game.SHEETX[index77] * num6, this.game.SHEETY[index77] * num7, num6, num7);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx, ty, num6, num7);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local81, ref local82, srcrect, destrect);
                        }
                        else
                        {
                          let mut nr8: i32 =  this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index75].FirstDrawOther].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                          let mut num84: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr8, Zoom) /  num6);
                          let mut num85: i32 =  0;
                          coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index74 + 1);
                          if (num84 > 1)
                            num85 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num84 - 1) : new Random((int) Math.Round(a1)).Next(0, num84 - 1);
                          ref Graphics local83 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(nr8, Zoom);
                          ref local84: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(num85 * num6, 0, num6, num7);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx, ty, num6, num7);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local83, ref local84, srcrect, destrect);
                          numArray31 = new int[6];
                        }
                      }
                      let mut index78: i32 =  0;
                      do
                      {
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index78] == this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index74])
                        {
                          numArray31[index78] = 1;
                          numArray32[index78] = 1;
                        }
                        else if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index78] > -1)
                        {
                          if (this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index78]].Category == this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index74]].Category & this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index74]].Category > -1)
                          {
                            numArray31[index78] = 1;
                            numArray32[index78] = 1;
                          }
                          else
                            numArray31[index78] = 0;
                        }
                        else
                          numArray31[index78] = 0;
                        index78 += 1;
                      }
                      while (index78 <= 5);
                      if (this.game.Data.RoadTypeObj[index75].UseSheet)
                      {
                        let mut sheetSpriteId: i32 =  this.game.Data.RoadTypeObj[index75].SheetSpriteID;
                        let mut index79: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                        ref Graphics local85 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                        ref local86: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(this.game.SHEETX[index79] * num6, this.game.SHEETY[index79] * num7, num6, num7);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx, ty, num6, num7);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local85, ref local86, srcrect, destrect);
                      }
                      else
                      {
                        let mut nr9: i32 =  this.game.Data.RoadTypeObj[index75].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                        let mut num86: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr9, Zoom) /  num6);
                        let mut num87: i32 =  0;
                        coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index74 + 1);
                        if (num86 > 1)
                          num87 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num86 - 1) : new Random((int) Math.Round(a1)).Next(0, num86 - 1);
                        ref Graphics local87 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(nr9, Zoom);
                        ref local88: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(num87 * num6, 0, num6, num7);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx, ty, num6, num7);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local87, ref local88, srcrect, destrect);
                      }
                    }
                  }
                }
                index74 += 1;
              }
              while (index74 <= 5);
            }
          }
          if ( this.game.Data.RuleVar[908] <= 0.0)
          {
            index63 = this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
            if (!this.game.EditObj.skipGfxDetail && this.game.AllowHeightMap & !(this.game.Data.LandscapeTypeObj[index63].IsSea & this.game.Data.LandscapeTypeObj[index63].Interior))
              this.DrawHeightMap(ref toG, cx, cy, cmap, tx, ty, Zoom, false, ref gBitmap);
          }
          if ( this.game.Data.RuleVar[32] > -1.0 &  this.game.Data.RuleVar[908] < 1.0)
          {
            numArray32 = new int[7];
            numArray31 = new int[7];
            bool flag12 = false;
            let mut index80: i32 =  0;
            do
            {
              let mut index81: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index80];
              if (index81 > -1 & numArray32[index80] == 0 && !this.game.Data.RiverTypeObj[index81].Transparent)
              {
                if (index81 == 4 & !flag12)
                {
                  flag12 = true;
                  this.DrawCanyon(ref toG, cx, cy, cmap, tx, ty, Zoom, ref gBitmap);
                }
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index80] > -1 & !this.game.Data.RiverTypeObj[index81].SpecialLayer | this.game.Data.RiverTypeObj[index81].SpecialLayer)
                {
                  nr10: i32;
                  if (!this.game.Data.RiverTypeObj[index81].SpecialLayer)
                  {
                    nr10 = this.game.Data.RiverTypeObj[index81].BasicSpriteID[index80];
                  }
                  else
                  {
                    let mut index82: i32 =  0;
                    do
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index82] == this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index80])
                      {
                        numArray31[index82] = 1;
                        numArray32[index82] = 1;
                      }
                      else
                        numArray31[index82] = 0;
                      index82 += 1;
                    }
                    while (index82 <= 5);
                    nr10 = this.game.Data.RiverTypeObj[index81].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                  }
                  if (this.game.Data.RiverTypeObj[index81].UseSheet)
                  {
                    let mut sheetSpriteId: i32 =  this.game.Data.RiverTypeObj[index81].SheetSpriteID;
                    let mut index83: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                    ref Graphics local89 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                    ref local90: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index83] * num6, this.game.SHEETY[index83] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local89, ref local90, srcrect, destrect);
                  }
                  else
                  {
                    let mut num88: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr10, Zoom) /  num6);
                    let mut num89: i32 =  0;
                    if (!this.game.Data.RiverTypeObj[index81].snakeMode)
                    {
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index80 + 1);
                      if (num88 > 1)
                        num89 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num88 - 1) : new Random((int) Math.Round(a1)).Next(0, num88 - 1);
                    }
                    else
                    {
                      index55 = index80 - 1;
                      let mut index84: i32 =  index80 + 1;
                      if (index55 < 0)
                        index55 = 5;
                      if (index84 > 5)
                        index84 = 0;
                      index8 = this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index55];
                      let mut num90: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index84];
                      if (index8 == index81 & num90 == index81)
                        num89 = 0;
                      if (index8 == index81 & num90 != index81)
                        num89 = 1;
                      if (index8 != index81 & num90 == index81)
                        num89 = 2;
                      if (index8 != index81 & num90 != index81)
                        num89 = 3;
                    }
                    ref Graphics local91 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr10, Zoom);
                    ref local92: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num89 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local91, ref local92, srcrect, destrect);
                  }
                }
              }
              index80 += 1;
            }
            while (index80 <= 5);
          }
          if (index42 > -1)
          {
            let mut num91: i32 =  index42;
            for (let mut index85: i32 =  0; index85 <= num91; index85 += 1)
            {
              if (numArray8[index85] < 1 & numArray9[index85] >= 899)
              {
                if (flagArray1[index85])
                {
                  let mut index86: i32 =  0;
                  do
                  {
                    if (numArray12[index85, index86] > 0 && numArray9[index85] <= 900)
                    {
                      let mut num92: i32 =  (int) Math.Round( BitmapStore.GetWidth(numArray12[index85, index86], Zoom) /  num6);
                      let mut num93: i32 =  0;
                      if (num92 > 1)
                      {
                        Random random = new Random((int) Math.Round(a1));
                        if (flagArray2[index85])
                          random = new Random((int) Math.Round(a2));
                        num93 = random.Next(0, num92 - 1);
                      }
                      ref Graphics local93 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(numArray12[index85, index86], Zoom);
                      ref local94: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num93 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local93, ref local94, srcrect, destrect);
                    }
                    index86 += 1;
                  }
                  while (index86 <= 5);
                }
                else if (numArray9[index85] <= 900)
                {
                  if (numArray10[index85] > -1)
                  {
                    let mut nr11: i32 =  numArray10[index85];
                    let mut index87: i32 =  numArray11[index85];
                    ref Graphics local95 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr11, Zoom);
                    ref local96: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index87] * num6, this.game.SHEETY[index87] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local95, ref local96, srcrect, destrect);
                  }
                  else
                  {
                    let mut maxValue: i32 =  (int) Math.Round( BitmapStore.GetWidth(numArray7[index85], Zoom) /  num6);
                    let mut num94: i32 =  0;
                    if (maxValue > 1)
                    {
                      Random random = new Random((int) Math.Round(a1));
                      if (flagArray2[index85])
                        random = new Random((int) Math.Round(a2));
                      num94 = random.Next(0, maxValue);
                    }
                    if (this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule > 0)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule <= maxValue)
                      {
                        num94 = this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule - 1;
                      }
                      else
                      {
                        this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule = 1;
                        num94 = this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule - 1;
                      }
                    }
                    ref Graphics local97 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(numArray7[index85], Zoom);
                    ref local98: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num94 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local97, ref local98, srcrect, destrect);
                  }
                }
              }
            }
          }
          if (index63 > -1 && this.game.Data.LandscapeTypeObj[index63].BasicSpriteCounter >= index64 && this.game.Data.LandscapeTypeObj[index63].PlotBeforeRiver[index64] && index64 > -1)
          {
            nr1 = this.game.Data.LandscapeTypeObj[index63].BasicSpriteID2[index64];
            let mut num95: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
            let mut num96: i32 =  0;
            if (num95 > 1)
              num96 = new Random((int) Math.Round(a1)).Next(0, num95 - 1);
            ref Graphics local99 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
            ref local100: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(num96 * num6, 0, num6, num7);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx, ty, num6, num7);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local99, ref local100, srcrect, destrect);
          }
          if ( this.game.Data.RuleVar[908] > 0.0)
          {
            let mut landscapeType2: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
            if (!this.game.EditObj.skipGfxDetail && this.game.AllowHeightMap)
              this.DrawHeightMap(ref toG, cx, cy, cmap, tx, ty, Zoom, false, ref gBitmap);
          }
          if (index42 > -1)
          {
            let mut num97: i32 =  index42;
            for (let mut index88: i32 =  0; index88 <= num97; index88 += 1)
            {
              if (numArray8[index88] > 0)
              {
                if (flagArray1[index88])
                {
                  let mut index89: i32 =  0;
                  do
                  {
                    if (numArray12[index88, index89] > 0 && numArray9[index88] <= 900)
                    {
                      let mut num98: i32 =  (int) Math.Round( BitmapStore.GetWidth(numArray12[index88, index89], Zoom) /  num6);
                      let mut num99: i32 =  0;
                      if (num98 > 1)
                      {
                        Random random = new Random((int) Math.Round(a1));
                        if (flagArray2[index88])
                          random = new Random((int) Math.Round(a2));
                        num99 = random.Next(0, num98 - 1);
                      }
                      ref Graphics local101 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(numArray12[index88, index89], Zoom);
                      ref local102: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num99 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local101, ref local102, srcrect, destrect);
                    }
                    index89 += 1;
                  }
                  while (index89 <= 5);
                }
                else if (numArray9[index88] <= 900)
                {
                  if (numArray10[index88] > -1)
                  {
                    let mut nr12: i32 =  numArray10[index88];
                    let mut index90: i32 =  numArray11[index88];
                    ref Graphics local103 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr12, Zoom);
                    ref local104: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index90] * num6, this.game.SHEETY[index90] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local103, ref local104, srcrect, destrect);
                  }
                  else
                  {
                    let mut maxValue: i32 =  (int) Math.Round( BitmapStore.GetWidth(numArray7[index88], Zoom) /  num6);
                    let mut num100: i32 =  0;
                    if (maxValue > 1)
                    {
                      Random random = new Random((int) Math.Round(a1));
                      if (flagArray2[index88])
                        random = new Random((int) Math.Round(a2));
                      num100 = random.Next(0, maxValue);
                    }
                    if (this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule > 0)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule <= maxValue)
                      {
                        num100 = this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule - 1;
                      }
                      else
                      {
                        this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule = 1;
                        num100 = this.game.Data.MapObj[cmap].HexObj[cx, cy].randomOverrule - 1;
                      }
                    }
                    ref Graphics local105 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(numArray7[index88], Zoom);
                    ref local106: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num100 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local105, ref local106, srcrect, destrect);
                  }
                }
              }
            }
          }
          if ( this.game.Data.RuleVar[908] > 0.0)
          {
            numArray32 = new int[7];
            numArray31 = new int[7];
            bool flag13 = false;
            bool flag14 = false;
            let mut index91: i32 =  0;
            do
            {
              let mut index92: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index91];
              if (index92 == -1)
              {
                let mut index93: i32 =  index91 + 1;
                if (index93 > 5)
                  index93 = 0;
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index93] == -1)
                {
                  if (index91 == 0)
                  {
                    index55 = 0;
                    index93 = 2;
                  }
                  if (index91 == 1)
                  {
                    index55 = 1;
                    index93 = 3;
                  }
                  if (index91 == 2)
                  {
                    index55 = 2;
                    index93 = 4;
                  }
                  if (index91 == 3)
                  {
                    index55 = 3;
                    index93 = 5;
                  }
                  if (index91 == 4)
                  {
                    index55 = 4;
                    index93 = 0;
                  }
                  if (index91 == 5)
                  {
                    index55 = 5;
                    index93 = 1;
                  }
                  coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index55 + 1);
                  if (coordinate1.onmap)
                  {
                    index8 = this.game.Data.MapObj[cmap].HexObj[coordinate1.x, coordinate1.y].RiverType[index93];
                    if (index8 > -1)
                    {
                      if (!flag13)
                      {
                        flag13 = true;
                        this.DrawCanyon(ref toG, cx, cy, cmap, tx, ty, Zoom, ref gBitmap);
                      }
                      if (this.game.Data.RiverTypeObj[index8].snakeMode)
                      {
                        let mut nr13: i32 =  this.game.Data.RiverTypeObj[index8].BasicSpriteID[index91];
                        let mut num101: i32 =  4;
                        ref Graphics local107 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(nr13, Zoom);
                        ref local108: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(num101 * num6, 0, num6, num7);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx, ty, num6, num7);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local107, ref local108, srcrect, destrect);
                      }
                    }
                  }
                }
              }
              if (index92 > -1 & numArray32[index91] == 0 && !this.game.Data.RiverTypeObj[index92].Transparent)
              {
                if (index92 == 4 & !flag14)
                {
                  flag14 = true;
                  this.DrawCanyon(ref toG, cx, cy, cmap, tx, ty, Zoom, ref gBitmap);
                }
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index91] > -1 & !this.game.Data.RiverTypeObj[index92].SpecialLayer | this.game.Data.RiverTypeObj[index92].SpecialLayer)
                {
                  nr14: i32;
                  if (!this.game.Data.RiverTypeObj[index92].SpecialLayer)
                  {
                    nr14 = this.game.Data.RiverTypeObj[index92].BasicSpriteID[index91];
                  }
                  else
                  {
                    let mut index94: i32 =  0;
                    do
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index94] == this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index91])
                      {
                        numArray31[index94] = 1;
                        numArray32[index94] = 1;
                      }
                      else
                        numArray31[index94] = 0;
                      index94 += 1;
                    }
                    while (index94 <= 5);
                    nr14 = this.game.Data.RiverTypeObj[index92].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                  }
                  if (this.game.Data.RiverTypeObj[index92].UseSheet)
                  {
                    let mut sheetSpriteId: i32 =  this.game.Data.RiverTypeObj[index92].SheetSpriteID;
                    let mut index95: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                    ref Graphics local109 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                    ref local110: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index95] * num6, this.game.SHEETY[index95] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local109, ref local110, srcrect, destrect);
                  }
                  else
                  {
                    let mut num102: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr14, Zoom) /  num6);
                    let mut num103: i32 =  0;
                    if (!this.game.Data.RiverTypeObj[index92].snakeMode)
                    {
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index91 + 1);
                      if (num102 > 1)
                        num103 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num102 - 1) : new Random((int) Math.Round(a1)).Next(0, num102 - 1);
                    }
                    else
                    {
                      index55 = index91 - 1;
                      let mut index96: i32 =  index91 + 1;
                      if (index55 < 0)
                        index55 = 5;
                      if (index96 > 5)
                        index96 = 0;
                      index8 = this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index55];
                      let mut num104: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index96];
                      if (index8 == index92 & num104 == index92)
                        num103 = 0;
                      if (index8 == index92 & num104 != index92)
                        num103 = 1;
                      if (index8 != index92 & num104 == index92)
                        num103 = 2;
                      if (index8 != index92 & num104 != index92)
                        num103 = 3;
                    }
                    ref Graphics local111 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(nr14, Zoom);
                    ref local112: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(num103 * num6, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local111, ref local112, srcrect, destrect);
                  }
                }
              }
              index91 += 1;
            }
            while (index91 <= 5);
          }
          if ( this.game.Data.RuleVar[32] > -1.0)
          {
            bool flag15 = false;
            if (index5 > -1)
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0)
                flag15 = true;
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].MaxRecon > 0 & index5 == this.game.Data.Turn)
                flag15 = true;
            }
            else
              flag15 = true;
            if (!this.game.Data.FOWOn)
              flag15 = true;
            if (InfoMode)
              flag15 = false;
            if (flag15)
            {
              numArray32 = new int[7];
              numArray31 = new int[7];
              bool flag16 = false;
              let mut index97: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index97] > -1 & numArray32[index97] == 0)
                {
                  let mut index98: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index97];
                  if (!this.game.Data.RoadTypeObj[index98].Transparent && !this.game.Data.RoadTypeObj[index98].SpecialLayer && this.game.Data.RoadTypeObj[index98].useCenter6 & !flag16)
                  {
                    flag16 = true;
                    let mut center6spriteId: i32 =  this.game.Data.RoadTypeObj[index98].center6spriteId;
                    ref Graphics local113 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(center6spriteId, Zoom);
                    ref local114: Bitmap = ref bitmap1;
                    rectangle2 = Rectangle::new(0, 0, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local113, ref local114, srcrect, destrect);
                  }
                }
                index97 += 1;
              }
              while (index97 <= 5);
              let mut index99: i32 =  0;
              do
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99] > -1 & numArray32[index99] == 0)
                {
                  let mut index100: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99];
                  if (!this.game.Data.RoadTypeObj[index100].Transparent)
                  {
                    if (!this.game.Data.RoadTypeObj[index100].SpecialLayer)
                    {
                      let mut nr15: i32 =  this.game.Data.RoadTypeObj[index100].BasicSpriteID[index99];
                      let mut num105: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr15, Zoom) /  num6);
                      let mut num106: i32 =  0;
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index99 + 1);
                      if (num105 > 1)
                        num106 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num105 - 1) : new Random((int) Math.Round(a1)).Next(0, num105 - 1);
                      ref Graphics local115 = ref toG;
                      bitmap1 = BitmapStore.GetBitmap(nr15, Zoom);
                      ref local116: Bitmap = ref bitmap1;
                      rectangle2 = Rectangle::new(num106 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local115, ref local116, srcrect, destrect);
                    }
                    else
                    {
                      if (this.game.Data.RoadTypeObj[index100].FirstDrawOther > -1)
                      {
                        let mut index101: i32 =  0;
                        do
                        {
                          numArray31[index101] = this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index101] != this.game.Data.RoadTypeObj[index100].FirstDrawOther ? (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index101] != this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99] ? (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index101] <= -1 ? 0 : (!(this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index101]].Category == this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99]].Category & this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99]].Category > -1) ? (!(this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index101]].Category == this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index100].FirstDrawOther].Category & this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index100].FirstDrawOther].Category > -1) ? 0 : 1) : 1)) : 1) : 1;
                          index101 += 1;
                        }
                        while (index101 <= 5);
                        if (this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index100].FirstDrawOther].UseSheet)
                        {
                          let mut sheetSpriteId: i32 =  this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index100].FirstDrawOther].SheetSpriteID;
                          let mut index102: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                          ref Graphics local117 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                          ref local118: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(this.game.SHEETX[index102] * num6, this.game.SHEETY[index102] * num7, num6, num7);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx, ty, num6, num7);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local117, ref local118, srcrect, destrect);
                        }
                        else
                        {
                          let mut nr16: i32 =  this.game.Data.RoadTypeObj[this.game.Data.RoadTypeObj[index100].FirstDrawOther].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                          let mut num107: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr16, Zoom) /  num6);
                          let mut num108: i32 =  0;
                          coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index99 + 1);
                          if (num107 > 1)
                            num108 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num107 - 1) : new Random((int) Math.Round(a1)).Next(0, num107 - 1);
                          ref Graphics local119 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(nr16, Zoom);
                          ref local120: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(num108 * num6, 0, num6, num7);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx, ty, num6, num7);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local119, ref local120, srcrect, destrect);
                          numArray31 = new int[6];
                        }
                      }
                      let mut index103: i32 =  0;
                      do
                      {
                        let mut index104: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index103];
                        if (index104 == this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99])
                        {
                          numArray31[index103] = 1;
                          numArray32[index103] = 1;
                        }
                        else if (index104 == -1)
                          numArray31[index103] = 0;
                        else if (this.game.Data.RoadTypeObj[index104].FirstDrawOther == this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99] & index99 > index103)
                        {
                          numArray31[index103] = 1;
                          numArray32[index103] = 1;
                        }
                        else if (index104 > -1)
                        {
                          if (this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index99]].Category == this.game.Data.RoadTypeObj[index104].Category & this.game.Data.RoadTypeObj[index104].Category > -1)
                          {
                            numArray31[index103] = 1;
                            numArray32[index103] = 1;
                          }
                          else
                            numArray31[index103] = 0;
                        }
                        else
                          numArray31[index103] = 0;
                        index103 += 1;
                      }
                      while (index103 <= 5);
                      if (this.game.Data.RoadTypeObj[index100].UseSheet)
                      {
                        let mut sheetSpriteId: i32 =  this.game.Data.RoadTypeObj[index100].SheetSpriteID;
                        let mut index105: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                        ref Graphics local121 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                        ref local122: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(this.game.SHEETX[index105] * num6, this.game.SHEETY[index105] * num7, num6, num7);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx, ty, num6, num7);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local121, ref local122, srcrect, destrect);
                      }
                      else
                      {
                        let mut nr17: i32 =  this.game.Data.RoadTypeObj[index100].LayerSpriteID[this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]]];
                        let mut num109: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr17, Zoom) /  num6);
                        let mut num110: i32 =  0;
                        coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index99 + 1);
                        if (num109 > 1)
                          num110 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num109 - 1) : new Random((int) Math.Round(a1)).Next(0, num109 - 1);
                        ref Graphics local123 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(nr17, Zoom);
                        ref local124: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(num110 * num6, 0, num6, num7);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx, ty, num6, num7);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local123, ref local124, srcrect, destrect);
                      }
                    }
                  }
                }
                index99 += 1;
              }
              while (index99 <= 5);
            }
          }
          index1 = 0;
          do
          {
            let mut index106: i32 =  index1 + 3;
            if (index106 > 5)
              index106 -= 6;
            coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index1 + 1);
            bool flag17 = false;
            if (index5 > -1 &&  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_ReconPts(index5) >=  this.game.Data.RuleVar[8] | !this.game.Data.FOWOn)
              flag17 = true;
            if (this.game.EditObj.RealRound == 0 | !this.game.Data.FOWOn)
              flag17 = true;
            if (flag17 & !InfoMode)
            {
              bool flag18 = false;
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1] > -1 &&  this.game.Data.RiverTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1]].BridgeCostModifier < 0.0)
                flag18 = true;
              if (!flag18)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Bridge[index1])
                {
                  if (cx == 6 & cy == 11)
                    cx = cx;
                  let mut index107: i32 =  -1;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1] > -1 && this.game.Data.RoadTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1]].BridgeOverrule)
                    index107 = this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1];
                  if (index107 > -1)
                  {
                    ref Graphics local125 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(this.game.Data.RoadTypeObj[index107].BridgeOverruleSpriteID[index1], Zoom);
                    ref local126: Bitmap = ref bitmap1;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    DrawMod.DrawSimple(ref local125, ref local126, x, y);
                    index1 = index1;
                  }
                  else
                  {
                    ref Graphics local127 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(this.game.Data.BridgeObj[0].BasicSpriteID[index1], Zoom);
                    ref local128: Bitmap = ref bitmap1;
                    let mut x1: i32 =  tx;
                    let mut y1: i32 =  ty;
                    DrawMod.DrawSimple(ref local127, ref local128, x1, y1);
                    if (coordinate1.onmap)
                    {
                      if (this.game.Data.BridgeObj[0].AlternateIfRoadType > -1 & this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1] == this.game.Data.BridgeObj[0].AlternateIfRoadType)
                      {
                        if (this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].RoadType[index106] == this.game.Data.BridgeObj[0].AlternateIfRoadType)
                        {
                          ref Graphics local129 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(this.game.Data.BridgeObj[0].AlternateSpriteID[index1], Zoom);
                          ref local130: Bitmap = ref bitmap1;
                          let mut x2: i32 =  tx;
                          let mut y2: i32 =  ty;
                          DrawMod.DrawSimple(ref local129, ref local130, x2, y2);
                        }
                      }
                      else if (this.game.Data.BridgeObj[0].AlternateIfRoadType2 > -1 & this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1] == this.game.Data.BridgeObj[0].AlternateIfRoadType2 && this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].RoadType[index106] == this.game.Data.BridgeObj[0].AlternateIfRoadType2)
                      {
                        ref Graphics local131 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(this.game.Data.BridgeObj[0].AlternateSpriteID[index1], Zoom);
                        ref local132: Bitmap = ref bitmap1;
                        let mut x3: i32 =  tx;
                        let mut y3: i32 =  ty;
                        DrawMod.DrawSimple(ref local131, ref local132, x3, y3);
                      }
                    }
                  }
                }
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RoadType[index1] > -1 & this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1] > -1 & !this.game.Data.MapObj[cmap].HexObj[cx, cy].Bridge[index1] &&  this.game.Data.RuleVar[308] < 1.0)
                {
                  coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1 + 1);
                  if (coordinate1.onmap && this.game.Data.MapObj[cmap].HexObj[coordinate1.x, coordinate1.y].RoadType[this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, coordinate1.map, cx, cy, cmap) - 1] > -1 &&  this.game.Data.RuleVar[32] > -1.0)
                  {
                    ref Graphics local133 = ref toG;
                    bitmap1 = BitmapStore.GetBitmap(this.game.NOBRIDGE[index1], Zoom);
                    ref local134: Bitmap = ref bitmap1;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    DrawMod.DrawSimple(ref local133, ref local134, x, y);
                  }
                }
              }
            }
            index1 += 1;
          }
          while (index1 <= 5);
          index6 = this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType;
          let mut index108: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].SpriteNr;
          if (index5 > -1)
          {
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              index6 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) == 0)
              index108 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastSpr(index5);
          }
          if (index6 > -1)
          {
            if (this.game.Data.LandscapeTypeObj[index6].BasicSpriteCounter >= index108)
            {
              if (this.game.Data.LandscapeTypeObj[index6].PlotLast[index108])
              {
                if (index108 > -1)
                {
                  nr1 = this.game.Data.LandscapeTypeObj[index6].BasicSpriteID2[index108];
                  let mut num111: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
                  let mut num112: i32 =  0;
                  if (num111 > 1)
                    num112 = new Random((int) Math.Round(a1)).Next(0, num111 - 1);
                  ref Graphics local135 = ref toG;
                  bitmap1 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local136: Bitmap = ref bitmap1;
                  rectangle2 = Rectangle::new(num112 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local135, ref local136, srcrect, destrect);
                }
                else
                  nr1 = this.game.NOHEX;
              }
            }
            else
              nr1 = this.game.NOHEX;
          }
          else
            nr1 = this.game.NOHEX;
          if (this.game.AllowHeightMap && !this.game.EditObj.skipGfxDetail)
            this.DrawHeightMapLate(ref toG, cx, cy, cmap, tx, ty, Zoom);
          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == index5 & this.game.EditObj.RealRound > 0 & !flag1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].Location > -1)
          {
            let mut hq1: i32 =  this.game.Data.LocObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Location].HQ;
          }
          if ( this.game.Data.RuleVar[401] > 0.0 & this.game.Data.Round > 0 & !flag1 & !InfoMode)
          {
            let mut strId123slot: i32 =  this.strId123slot;
            let mut strId143slot: i32 =  this.strId143slot;
            let mut strId288slot: i32 =  this.strId288slot;
            if (this.game.Data.TempString[742].Length > 0 & this.game.Data.TempString[743].Length > 0)
            {
              let mut idValue1: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(cx, cy, this.game.Data.TempString[742], this.game.Data.TempString[743])));
              if (idValue1 > 0)
              {
                let mut num113: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId123slot].GetData(0, idValue1, 8)));
                index1 = 1;
                do
                {
                  coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                  if (coordinate2.onmap)
                  {
                    bool flag19 = false;
                    if (index5 > -1)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].get_SeeNow(index5) > 0 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0 | !this.game.Data.FOWOn)
                        flag19 = true;
                    }
                    else if (!this.game.Data.FOWOn)
                      flag19 = true;
                    if (!flag19)
                    {
                      let mut index109: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(coordinate2.x, coordinate2.y, this.game.Data.TempString[742], this.game.Data.TempString[743])));
                      if (index109 > 0 & index5 > -1)
                      {
                        index8 = this.cacheZoneRecon[this.game.Data.RegimeObj[index5].id, idValue1];
                        let mut num114: i32 =  this.cacheZoneRecon[this.game.Data.RegimeObj[index5].id, index109];
                        if (index8 > 0 | num114 > 0)
                          flag19 = true;
                      }
                    }
                    if (flag19)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime == this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime)
                      {
                        if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate2.map].HexObj[coordinate2.x, coordinate2.y].LandscapeType].BlackedOut && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType].IsSea == this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea)
                        {
                          let mut idValue2: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(coordinate2.x, coordinate2.y, this.game.Data.TempString[742], this.game.Data.TempString[743])));
                          let mut num115: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId123slot].GetData(0, idValue2, 8)));
                          if (idValue1 != idValue2)
                          {
                            if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime == index5)
                            {
                              ref Graphics local137 = ref toG;
                              bitmap1 = BitmapStore.GetBitmap(this.game.LIGHTZONEBORDER[index1 - 1], Zoom);
                              ref local138: Bitmap = ref bitmap1;
                              rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                              let mut srcrect: &Rectangle = &rectangle2
                              rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                              let mut destrect: &Rectangle = &rectangle1
                              DrawMod.DrawSimplePart2(ref local137, ref local138, srcrect, destrect);
                            }
                            else
                            {
                              ref Graphics local139 = ref toG;
                              bitmap1 = BitmapStore.GetBitmap(this.game.LIGHTZONEBORDER[index1 - 1], Zoom);
                              ref local140: Bitmap = ref bitmap1;
                              rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                              let mut srcrect: &Rectangle = &rectangle2
                              rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                              let mut destrect: &Rectangle = &rectangle1
                              DrawMod.DrawSimplePart2(ref local139, ref local140, srcrect, destrect);
                            }
                          }
                        }
                      }
                      else if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime > -1 & this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime > -1 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate2.map].HexObj[coordinate2.x, coordinate2.y].LandscapeType].BlackedOut && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType].IsSea == this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea)
                      {
                        let mut idValue3: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(coordinate2.x, coordinate2.y, this.game.Data.TempString[742], this.game.Data.TempString[743])));
                        let mut num116: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId123slot].GetData(0, idValue3, 8)));
                        if (num113 == num116 & idValue1 != idValue3 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId143slot].GetData(0, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].id, 1))) > 1 & (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId143slot].GetData(0, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].id, 1))) > 1)
                        {
                          ref Graphics local141 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(this.game.LIGHTZONEBORDER[index1 - 1], Zoom);
                          ref local142: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local141, ref local142, srcrect, destrect);
                        }
                      }
                    }
                  }
                  index1 += 1;
                }
                while (index1 <= 6);
              }
            }
          }
          let mut num117: i32 =  0;
          if (!InfoMode & index6 > -1)
          {
            if (!flag1)
            {
              if (!ispredrawing && !this.game.Data.LandscapeTypeObj[index6].IsSea)
              {
                index1 = 1;
                do
                {
                  coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                  if (coordinate2.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea)
                  {
                    if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime != this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime)
                      num117 = 1;
                    if (this.game.Data.ShrowdOn & this.game.EditObj.RealRound > 0 && this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].get_LastReg(index5) != this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastReg(index5))
                      num117 = 1;
                  }
                  index1 += 1;
                }
                while (index1 <= 6);
              }
            }
            else if (!this.game.Data.LandscapeTypeObj[index6].IsSea)
            {
              index1 = 1;
              do
              {
                coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                if (coordinate2.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea && this.game.EditObj.HisOwner[coordinate2.map].Value[coordinate2.x, coordinate2.y] != this.game.EditObj.HisOwner[cmap].Value[cx, cy])
                  num117 = 1;
                index1 += 1;
              }
              while (index1 <= 6);
            }
          }
          if (num117 == 1 & !InfoMode)
          {
            if (!flag1)
            {
              if (!this.game.Data.LandscapeTypeObj[index6].IsSea & !this.game.Data.LandscapeTypeObj[index6].BlackedOut)
              {
                let mut strId143slot: i32 =  this.strId143slot;
                let mut strId275slot: i32 =  this.strId275slot;
                let mut strId288slot: i32 =  this.strId288slot;
                index1 = 1;
                do
                {
                  coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                  if (coordinate2.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate2.map].HexObj[coordinate2.x, coordinate2.y].LandscapeType].BlackedOut && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea)
                  {
                    bool flag20 = false;
                    if (index5 > -1)
                    {
                      if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].get_SeeNow(index5) > 0 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0 | !this.game.Data.FOWOn)
                        flag20 = true;
                    }
                    else if (!this.game.Data.FOWOn)
                      flag20 = true;
                    if (!flag20)
                    {
                      let mut index110: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(cx, cy, this.game.Data.TempString[742], this.game.Data.TempString[743])));
                      let mut index111: i32 =  (int) Math.Round(Conversion.Val(this.game.EventRelatedObj.CheckLibVarHex(coordinate2.x, coordinate2.y, this.game.Data.TempString[742], this.game.Data.TempString[743])));
                      if (index111 > 0 & index5 > -1)
                      {
                        index8 = this.cacheZoneRecon[this.game.Data.RegimeObj[index5].id, index110];
                        let mut num118: i32 =  this.cacheZoneRecon[this.game.Data.RegimeObj[index5].id, index111];
                        if (index8 > 0 | num118 > 0)
                          flag20 = true;
                      }
                    }
                    if (flag20 && this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime != this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime && !(this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime == -1 | this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == -1))
                    {
                      let mut regime1: i32 =  this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime;
                      let mut regime2: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime;
                      let mut num119: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId143slot].GetData(0, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].id, 1)));
                      let mut num120: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[strId143slot].GetData(0, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].id, 1)));
                      let mut num121: i32 =  this.cacheDipClear[this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].id, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].id];
                      let mut num122: i32 =  this.cacheDipClear[this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].id, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].id];
                      bool flag21 = true;
                      if (num119 == 1 & num120 == 1)
                        flag21 = false;
                      if (num119 > 1 & num121 > 0 | num121 > 1 & num122 > 0)
                        flag21 = false;
                      if (num120 > 1 & num119 == 1 & num121 > 0)
                        flag21 = false;
                      if (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime == -1 | this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == -1)
                      {
                        ref Graphics local143 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                        ref local144: Bitmap = ref bitmap1;
                        let mut x: i32 =  tx;
                        let mut y: i32 =  ty;
                        DrawMod.DrawSimple(ref local143, ref local144, x, y);
                      }
                      else if (flag21 & index5 > -1 & (this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime == index5 | this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == index5))
                      {
                        ref Graphics local145 = ref toG;
                        bitmap1 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                        ref local146: Bitmap = ref bitmap1;
                        rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                        let mut srcrect: &Rectangle = &rectangle2
                        rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                        let mut destrect: &Rectangle = &rectangle1
                        DrawMod.DrawSimplePart2(ref local145, ref local146, srcrect, destrect);
                      }
                      else
                      {
                        if (num119 == 4)
                          num121 = 0;
                        if (num120 == 4)
                          num122 = 0;
                        if (num119 > 1 & num121 < 1 & num120 > 1 & num122 < 1)
                        {
                          ref Graphics local147 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(this.game.LIGHTZONEBORDER[index1 - 1], Zoom);
                          ref local148: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local147, ref local148, srcrect, destrect);
                        }
                        else if (index5 == -1)
                        {
                          ref Graphics local149 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local150: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          DrawMod.DrawSimplePart2(ref local149, ref local150, srcrect, destrect);
                        }
                        else if (num119 > 1 & num121 < 1)
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local151 = ref toG;
                          bitmap1 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local152: Bitmap = ref bitmap1;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap4 = (Bitmap) null;
                          ref local153: Bitmap = ref bitmap4;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local151, ref local152, srcrect, destrect,  r,  g,  b, 0.45f, ref local153);
                        }
                        else if (num120 > 1 & num122 < 1)
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local154 = ref toG;
                          bitmap4 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local155: Bitmap = ref bitmap4;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap1 = (Bitmap) null;
                          ref local156: Bitmap = ref bitmap1;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local154, ref local155, srcrect, destrect,  r,  g,  b, 0.45f, ref local156);
                        }
                        else if (num119 == 1 & num120 > 1)
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local157 = ref toG;
                          bitmap4 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local158: Bitmap = ref bitmap4;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap1 = (Bitmap) null;
                          ref local159: Bitmap = ref bitmap1;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local157, ref local158, srcrect, destrect,  r,  g,  b, 0.45f, ref local159);
                        }
                        else if (num120 == 1 & num119 > 1)
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local160 = ref toG;
                          bitmap4 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local161: Bitmap = ref bitmap4;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap1 = (Bitmap) null;
                          ref local162: Bitmap = ref bitmap1;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local160, ref local161, srcrect, destrect,  r,  g,  b, 0.45f, ref local162);
                        }
                        else if (regime2 == index5 | regime1 < regime2 & regime1 != index5)
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local163 = ref toG;
                          bitmap4 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local164: Bitmap = ref bitmap4;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap1 = (Bitmap) null;
                          ref local165: Bitmap = ref bitmap1;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local163, ref local164, srcrect, destrect,  r,  g,  b, 0.45f, ref local165);
                        }
                        else
                        {
                          a3 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Red / 512.0 - 0.9);
                          a4 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Green / 512.0 - 0.9);
                          a5 =  ( this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Blue / 512.0 - 0.9);
                          ref Graphics local166 = ref toG;
                          bitmap4 = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                          ref local167: Bitmap = ref bitmap4;
                          rectangle2 = Rectangle::new(numArray13[index1], numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut srcrect: &Rectangle = &rectangle2
                          rectangle1 = Rectangle::new(tx + numArray13[index1], ty + numArray14[index1], numArray15[index1], numArray16[index1]);
                          let mut destrect: &Rectangle = &rectangle1
                          double r = 1.0 +  a3;
                          double g = 1.0 +  a4;
                          double b = 1.0 +  a5;
                          bitmap1 = (Bitmap) null;
                          ref local168: Bitmap = ref bitmap1;
                          DrawMod.DrawSimplePart2ColouredNewFast(ref local166, ref local167, srcrect, destrect,  r,  g,  b, 0.45f, ref local168);
                        }
                      }
                    }
                  }
                  index1 += 1;
                }
                while (index1 <= 6);
              }
            }
            else if (!this.game.Data.LandscapeTypeObj[index6].IsSea & !this.game.Data.LandscapeTypeObj[index6].BlackedOut)
            {
              index1 = 1;
              do
              {
                coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                if (coordinate2.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate2.map].HexObj[coordinate2.x, coordinate2.y].LandscapeType].BlackedOut && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate2.x, coordinate2.y].LandscapeType].IsSea && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.EditObj.HisOwner[cmap].Value[coordinate2.x, coordinate2.y], this.game.EditObj.HisOwner[cmap].Value[cx, cy]) && !(this.game.EditObj.HisOwner[cmap].Value[coordinate2.x, coordinate2.y] <= -1 & this.game.EditObj.HisOwner[cmap].Value[cx, cy] <= -1) && !(this.game.EditObj.HisOwner[cmap].Value[coordinate2.x, coordinate2.y] == -2 | this.game.EditObj.HisOwner[cmap].Value[cx, cy] == -2))
                {
                  ref Graphics local169 = ref toG;
                  bitmap5: Bitmap = BitmapStore.GetBitmap(this.game.BORDER[index1 - 1], Zoom);
                  ref local170: Bitmap = ref bitmap5;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.DrawSimple(ref local169, ref local170, x, y);
                }
                index1 += 1;
              }
              while (index1 <= 6);
            }
          }
          if (index42 > -1)
          {
            let mut num123: i32 =  index42;
            for (index1 = 0; index1 <= num123; index1 += 1)
            {
              if (numArray9[index1] > 900)
              {
                let mut num124: i32 =  (int) Math.Round( BitmapStore.GetWidth(numArray7[index1], Zoom) /  num6);
                let mut num125: i32 =  0;
                if (num124 > 1)
                {
                  Random random = new Random((int) Math.Round(a1));
                  if (flagArray2[index1])
                    random = new Random((int) Math.Round(a2));
                  num125 = random.Next(0, num124 - 1);
                }
                ref Graphics local171 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(numArray7[index1], Zoom);
                ref local172: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num125 * num6, 0, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local171, ref local172, srcrect, destrect);
              }
            }
          }
          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Location > -1 & !InfoMode)
          {
            bool flag22 = false;
            if (index5 > -1)
            {
              if (!this.game.Data.FOWOn | this.game.Data.Turn == index5 & this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 & !flag1 | this.game.EditObj.RealRound < 1 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) > 0)
                flag22 = true;
            }
            else
              flag22 = true;
            if (flag22)
            {
              let mut type: i32 =  this.game.Data.LocObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Location].Type;
              if (this.game.Data.MapObj[0].HexObj[cx, cy].Regime > -1 & this.game.Data.LocTypeObj[type].SmallGraphicSpecialMode == 1)
              {
                color: Color = Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Red, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Green, this.game.Data.RegimeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime].Blue);
                a3 =  ((int) color.R + 128) / 384f;
                a4 =  ((int) color.G + 128) / 384f;
                a5 =  ((int) color.B + 128) / 384f;
                let mut nr18: i32 =  this.game.Data.SmallPicNr[this.game.Data.LocTypeObj[type].SmallGraphic];
                if (nr18 > -1)
                {
                  ref Graphics local173 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr18, Zoom);
                  ref local174: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(0, Math.Max(0, this.game.Data.LocTypeObj[type].SmallGraphicSpecialData) * num7, num6, num7);
                  let mut srcrect5: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect5: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local173, ref local174, srcrect5, destrect5);
                  ref Graphics local175 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr18, Zoom);
                  ref local176: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num6, Math.Max(0, this.game.Data.LocTypeObj[type].SmallGraphicSpecialData) * num7, num6, num7);
                  let mut srcrect6: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect6: &Rectangle = &rectangle1
                  double r =  a3;
                  double g =  a4;
                  double b =  a5;
                  DrawMod.DrawSimplePart2ColouredNew(ref local175, ref local176, srcrect6, destrect6,  r,  g,  b, 1f);
                }
              }
              else if (this.game.Data.LocTypeObj[type].SmallGraphicSpecialMode == 1)
              {
                let mut nr19: i32 =  this.game.Data.SmallPicNr[this.game.Data.LocTypeObj[type].SmallGraphic];
                if (nr19 > -1)
                {
                  ref Graphics local177 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr19, Zoom);
                  ref local178: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(0, Math.Max(0, this.game.Data.LocTypeObj[type].SmallGraphicSpecialData) * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local177, ref local178, srcrect, destrect);
                }
              }
              else if (this.game.Data.LocTypeObj[type].SmallGraphic > -1)
              {
                let mut nr20: i32 =  this.game.Data.SmallPicNr[this.game.Data.LocTypeObj[type].SmallGraphic];
                if (nr20 > -1)
                {
                  ref Graphics local179 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr20, Zoom);
                  ref local180: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(0, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local179, ref local180, srcrect, destrect);
                }
              }
              else if (this.game.Data.LocTypeObj[type].ExtraGraphic > -1)
              {
                let mut extraGraphic: i32 =  this.game.Data.LocTypeObj[type].ExtraGraphic;
                if (extraGraphic > -1)
                {
                  nr1 = this.game.NATO[extraGraphic];
                  ref Graphics local181 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local182: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(0, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local181, ref local182, srcrect, destrect);
                }
              }
            }
          }
          bool flag23 = false;
          if (index5 > -1)
          {
            if (this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(index5) > 0 | !this.game.Data.FOWOn | this.game.Data.Turn == index5 & this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 & !flag1)
              flag23 = true;
          }
          else
            flag23 = true;
          if (InfoMode)
            flag23 = false;
          if (flag23)
          {
            let mut regime: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime;
            if (regime > -1)
            {
              color: Color = DrawMod.LightenColor(Color.FromArgb((int) byte.MaxValue, this.game.Data.RegimeObj[regime].Red, this.game.Data.RegimeObj[regime].Green, this.game.Data.RegimeObj[regime].Blue), 50);
              a3 =  ((int) color.R + 511) / 767f;
              a4 =  ((int) color.G + 511) / 767f;
              a5 =  ((int) color.B + 511) / 767f;
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialType2 <= -1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite2 > -1)
            {
              nr1 = this.game.Data.SmallPicNr[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite2];
              let mut num126: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
              let mut num127: i32 =  0;
              if (num126 > 1)
                num127 = new Random((int) Math.Round(a1)).Next(0, num126 - 1);
              if (BitmapStore.Getheight(nr1, Zoom) > num7)
              {
                if (this.game.Data.MapObj[0].HexObj[cx, cy].Location == -1)
                {
                  ref Graphics local183 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local184: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num127 * num6, 0, num6, num7);
                  let mut srcrect7: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect7: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local183, ref local184, srcrect7, destrect7);
                  ref Graphics local185 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local186: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num127 * num6, num7, num6, num7);
                  let mut srcrect8: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect8: &Rectangle = &rectangle1
                  double r =  a3;
                  double g =  a4;
                  double b =  a5;
                  DrawMod.DrawSimplePart2ColouredNew(ref local185, ref local186, srcrect8, destrect8,  r,  g,  b, 1f);
                }
              }
              else
              {
                ref Graphics local187 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local188: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num127 * num6, 0, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local187, ref local188, srcrect, destrect);
              }
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialType3 <= -1 && this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite3 > -1)
            {
              nr1 = this.game.Data.SmallPicNr[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite3];
              let mut num128: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
              let mut num129: i32 =  0;
              if (num128 > 1)
                num129 = new Random((int) Math.Round(a1)).Next(0, num128 - 1);
              if (BitmapStore.Getheight(nr1, Zoom) > num7)
              {
                if (this.game.Data.MapObj[0].HexObj[cx, cy].Location == -1)
                {
                  ref Graphics local189 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local190: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num129 * num6, 0, num6, num7);
                  let mut srcrect9: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect9: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local189, ref local190, srcrect9, destrect9);
                  ref Graphics local191 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local192: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num129 * num6, num7, num6, num7);
                  let mut srcrect10: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect10: &Rectangle = &rectangle1
                  double r =  a3;
                  double g =  a4;
                  double b =  a5;
                  DrawMod.DrawSimplePart2ColouredNew(ref local191, ref local192, srcrect10, destrect10,  r,  g,  b, 1f);
                }
              }
              else
              {
                ref Graphics local193 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local194: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num129 * num6, 0, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local193, ref local194, srcrect, destrect);
              }
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialType > -1)
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite > -1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialType].BasicSpriteCounter >= this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite)
              {
                nr1 = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialType].BasicSpriteID[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite];
                let mut num130: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
                let mut num131: i32 =  0;
                if (num130 > 1)
                  num131 = new Random((int) Math.Round(a1)).Next(0, num130 - 1);
                if (BitmapStore.Getheight(nr1, Zoom) > num7)
                {
                  if (this.game.Data.MapObj[0].HexObj[cx, cy].Location == -1)
                  {
                    ref Graphics local195 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                    ref local196: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(num131 * num6, 0, num6, num7);
                    let mut srcrect11: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect11: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local195, ref local196, srcrect11, destrect11);
                    ref Graphics local197 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                    ref local198: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(num131 * num6, num7, num6, num7);
                    let mut srcrect12: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect12: &Rectangle = &rectangle1
                    double r =  a3;
                    double g =  a4;
                    double b =  a5;
                    DrawMod.DrawSimplePart2ColouredNew(ref local197, ref local198, srcrect12, destrect12,  r,  g,  b, 1f);
                  }
                }
                else
                {
                  ref Graphics local199 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                  ref local200: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num131 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local199, ref local200, srcrect, destrect);
                }
              }
            }
            else if (this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite > -1)
            {
              nr1 = this.game.Data.SmallPicNr[this.game.Data.MapObj[cmap].HexObj[cx, cy].SpecialSprite];
              let mut num132: i32 =  (int) Math.Round( BitmapStore.GetWidth(nr1, Zoom) /  num6);
              let mut num133: i32 =  0;
              if (num132 > 1)
                num133 = new Random((int) Math.Round(a1)).Next(0, num132 - 1);
              if (BitmapStore.Getheight(nr1, Zoom) > num7)
              {
                ref Graphics local201 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local202: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num133 * num6, 0, num6, num7);
                let mut srcrect13: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect13: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local201, ref local202, srcrect13, destrect13);
                ref Graphics local203 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local204: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num133 * num6, num7, num6, num7);
                let mut srcrect14: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect14: &Rectangle = &rectangle1
                double r =  a3;
                double g =  a4;
                double b =  a5;
                DrawMod.DrawSimplePart2ColouredNew(ref local203, ref local204, srcrect14, destrect14,  r,  g,  b, 1f);
              }
              else
              {
                ref Graphics local205 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
                ref local206: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num133 * num6, 0, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local205, ref local206, srcrect, destrect);
              }
            }
          }
          if (index5 > -1 & !InfoMode && this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(index5) > 0)
          {
            let mut cacheTad: i32 =  this.cache_tad;
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].GetHexLibVarValue(cacheTad) > 0)
            {
              let mut cacheTat: i32 =  this.cache_tat;
              let mut smallPic: i32 =  this.game.Data.FindSmallPic(this.game.Data.MapObj[cmap].HexObj[cx, cy].GetHexLibVarValue(cacheTat) + 157, "SE_Graphics");
              ref Graphics local207 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
              ref local208: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local207, ref local208, x, y);
            }
          }
          if (index5 > -1 && this.game.Data.MapObj[0].HexObj[cx, cy].get_SeeNow(index5) > 0 | index5 == this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 & !flag1 && index5 > -1 & !InfoMode && this.game.Data.SmallPicNr[72] > 0 && this.game.Data.MapObj[0].HexObj[cx, cy].Location > -1 && this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[cx, cy].Location].tempAirfieldLevel > 0)
          {
            ref Graphics local209 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[72], Zoom);
            ref local210: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local209, ref local210, x, y);
          }
          if (!flag1 & !InfoMode)
          {
            let mut cacheRad: i32 =  this.cache_rad;
            let mut hexLibVarValue: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].GetHexLibVarValue(cacheRad);
            if (hexLibVarValue > 50 & (this.game.Data.MapObj[0].HexObj[cx, cy].MaxRecon > 0 | !this.game.Data.FOWOn))
            {
              let mut num134: i32 =  hexLibVarValue;
              let mut num135: i32 =  0;
              if (num134 > 12800)
                num135 = 9;
              else if (num134 > 6400)
                num135 = 8;
              else if (num134 > 3200)
                num135 = 7;
              else if (num134 > 1600)
                num135 = 6;
              else if (num134 > 800)
                num135 = 5;
              else if (num134 > 400)
                num135 = 4;
              else if (num134 > 200)
                num135 = 3;
              else if (num134 > 100)
                num135 = 2;
              else if (num134 > 50)
                num135 = 1;
              let mut smallPic: i32 =  this.game.Data.FindSmallPic(162, "SE_Graphics");
              let mut num136: i32 =  (int) Math.Round( num6 / 2.0 * (Math.Sqrt( hexLibVarValue / 2.0) / 40.0));
              if ( num136 >  num6 / 2.0)
                num136 = (int) Math.Round( num6 / 2.0);
              if ( num136 <  num6 / 8.0)
                num136 = (int) Math.Round( num6 / 8.0);
              let mut num137: i32 =  (int) Math.Round(( num6 / 2.0 -  num136) / 2.0);
              let mut num138: i32 =  (int) Math.Round( num6 / 2.0);
              let mut num139: i32 =  num138;
              if (num135 <= 2)
              {
                ref Graphics local211 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local212: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local211, ref local212, x, y, w, h, origw, origh, 0.5f, 1f, 0.5f, 1f);
              }
              else if (num135 == 3)
              {
                ref Graphics local213 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local214: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local213, ref local214, x, y, w, h, origw, origh, 1f, 1f, 0.5f, 1f);
              }
              else if (num135 == 4)
              {
                ref Graphics local215 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local216: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local215, ref local216, x, y, w, h, origw, origh, 0.5f, 1f, 1f, 1f);
              }
              else if (num135 == 5)
              {
                ref Graphics local217 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local218: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local217, ref local218, x, y, w, h, origw, origh, 1f, 0.5f, 1f, 1f);
              }
              else if (num135 == 6)
              {
                ref Graphics local219 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local220: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local219, ref local220, x, y, w, h, origw, origh, 1f, 0.35f, 0.35f, 1f);
              }
              else if (num135 >= 7)
              {
                ref Graphics local221 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local222: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                let mut origw: i32 =  num138;
                let mut origh: i32 =  num139;
                DrawMod.DrawScaledColorized2(ref local221, ref local222, x, y, w, h, origw, origh, 1.5f, 0.0f, 0.0f, 1f);
              }
              else if (num135 >= 99)
              {
                ref Graphics local223 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], Zoom);
                ref local224: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + num137 + (int) Math.Round( num6 / 16.0);
                let mut y: i32 =  ty + num137;
                let mut w: i32 =  num136;
                let mut h: i32 =  num136;
                DrawMod.DrawScaled(ref local223, ref local224, x, y, w, h, true);
              }
            }
          }
          if (!InfoMode)
          {
            if (this.game.EditObj.LayerSupplyOn & !this.game.EditObj.AIMoving)
            {
              if (this.game.EditObj.TempSup[cmap].Value[cx, cy] < 9999)
              {
                if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType].IsSea)
                {
                  if ( this.game.EditObj.TempSup[cmap].Value[cx, cy] >  this.game.Data.RuleVar[3])
                  {
                    ref Graphics local225 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local226: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    DrawMod.Draw(ref local225, ref local226, x, y, -1f, -1f, -1f, 0.2f);
                  }
                  else if (this.game.EditObj.TempSup[cmap].Value[cx, cy] != 0)
                  {
                    if ( this.game.EditObj.TempSup[cmap].Value[cx, cy] >  this.game.Data.RuleVar[53])
                    {
                      ref Graphics local227 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                      ref local228: Bitmap = ref bitmap4;
                      let mut x: i32 =  tx;
                      let mut y: i32 =  ty;
                      double r = -0.5 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      DrawMod.Draw(ref local227, ref local228, x, y,  r, -1f, -1f, 0.2f);
                    }
                    else if ( this.game.EditObj.TempSup[cmap].Value[cx, cy] >  this.game.Data.RuleVar[52])
                    {
                      ref Graphics local229 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                      ref local230: Bitmap = ref bitmap4;
                      let mut x: i32 =  tx;
                      let mut y: i32 =  ty;
                      double g = -0.75 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      double b = -0.75 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      DrawMod.Draw(ref local229, ref local230, x, y, -1f,  g,  b, 0.2f);
                    }
                    else if ( this.game.EditObj.TempSup[cmap].Value[cx, cy] >  this.game.Data.RuleVar[51])
                    {
                      ref Graphics local231 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                      ref local232: Bitmap = ref bitmap4;
                      let mut x: i32 =  tx;
                      let mut y: i32 =  ty;
                      double r = -0.75 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      double g = -0.75 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      DrawMod.Draw(ref local231, ref local232, x, y,  r,  g, -1f, 0.2f);
                    }
                    else
                    {
                      ref Graphics local233 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                      ref local234: Bitmap = ref bitmap4;
                      let mut x: i32 =  tx;
                      let mut y: i32 =  ty;
                      double g = -0.75 * ( this.game.EditObj.TempSup[cmap].Value[cx, cy] /  this.game.Data.RuleVar[3]);
                      DrawMod.Draw(ref local233, ref local234, x, y, -1f,  g, -1f, 0.2f);
                    }
                  }
                  else
                  {
                    ref Graphics local235 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local236: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    DrawMod.Draw(ref local235, ref local236, x, y, 0.0f, 0.0f, 0.0f, 0.2f);
                  }
                }
                num140: i32;
                if (Information.IsNothing( this.game.EditObj.SupplyPath))
                  num140 = 0;
                else if (this.game.EditObj.SupplyPath.Exists(cx, cy, cmap))
                  num140 = 1;
                if (num140 == 1)
                {
                  coordinate1 = this.game.EditObj.TempSupCameFrom[cmap].Value[cx, cy];
                  switch (this.game.HandyFunctionsObj.HexFacing(cx, cy, cmap, coordinate1.x, coordinate1.y, coordinate1.map))
                  {
                    case 1:
                      ref Graphics local237 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK0, Zoom);
                      ref local238: Bitmap = ref bitmap4;
                      let mut x4: i32 =  tx;
                      let mut y4: i32 =  ty;
                      DrawMod.DrawSimple(ref local237, ref local238, x4, y4);
                      break;
                    case 2:
                      ref Graphics local239 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK1, Zoom);
                      ref local240: Bitmap = ref bitmap4;
                      let mut x5: i32 =  tx;
                      let mut y5: i32 =  ty;
                      DrawMod.DrawSimple(ref local239, ref local240, x5, y5);
                      break;
                    case 3:
                      ref Graphics local241 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK2, Zoom);
                      ref local242: Bitmap = ref bitmap4;
                      let mut x6: i32 =  tx;
                      let mut y6: i32 =  ty;
                      DrawMod.DrawSimple(ref local241, ref local242, x6, y6);
                      break;
                    case 4:
                      ref Graphics local243 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK3, Zoom);
                      ref local244: Bitmap = ref bitmap4;
                      let mut x7: i32 =  tx;
                      let mut y7: i32 =  ty;
                      DrawMod.DrawSimple(ref local243, ref local244, x7, y7);
                      break;
                    case 5:
                      ref Graphics local245 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK4, Zoom);
                      ref local246: Bitmap = ref bitmap4;
                      let mut x8: i32 =  tx;
                      let mut y8: i32 =  ty;
                      DrawMod.DrawSimple(ref local245, ref local246, x8, y8);
                      break;
                    case 6:
                      ref Graphics local247 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK5, Zoom);
                      ref local248: Bitmap = ref bitmap4;
                      let mut x9: i32 =  tx;
                      let mut y9: i32 =  ty;
                      DrawMod.DrawSimple(ref local247, ref local248, x9, y9);
                      break;
                  }
                }
              }
            }
            else if (this.game.EditObj.ShowTransfer & !this.game.EditObj.AIMoving)
            {
              if (this.game.EditObj.TempValue[cmap].Value[cx, cy] < 9999 & !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].LandscapeType].IsSea)
              {
                float num141 = this.game.Data.RuleVar[78] / this.game.Data.RuleVar[3];
                if (this.game.EditObj.TempValue[cmap].Value[cx, cy] > 0)
                {
                  if ( this.game.EditObj.TempValue[cmap].Value[cx, cy] >  this.game.Data.RuleVar[53] *  num141)
                  {
                    ref Graphics local249 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local250: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    double r = -0.5 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    DrawMod.Draw(ref local249, ref local250, x, y,  r, -1f, -1f, 0.2f);
                  }
                  else if ( this.game.EditObj.TempValue[cmap].Value[cx, cy] >  this.game.Data.RuleVar[52] *  num141)
                  {
                    ref Graphics local251 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local252: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    double g = -0.75 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    double b = -0.75 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    DrawMod.Draw(ref local251, ref local252, x, y, -1f,  g,  b, 0.2f);
                  }
                  else if ( this.game.EditObj.TempValue[cmap].Value[cx, cy] >  this.game.Data.RuleVar[51] *  num141)
                  {
                    ref Graphics local253 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local254: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    double r = -0.75 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    double g = -0.75 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    DrawMod.Draw(ref local253, ref local254, x, y,  r,  g, -1f, 0.2f);
                  }
                  else
                  {
                    ref Graphics local255 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                    ref local256: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    double g = -0.75 * ( this.game.EditObj.TempValue[cmap].Value[cx, cy] /  this.game.Data.RuleVar[78]);
                    DrawMod.Draw(ref local255, ref local256, x, y, -1f,  g, -1f, 0.2f);
                  }
                }
                else
                {
                  ref Graphics local257 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local258: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local257, ref local258, x, y, 0.0f, 0.0f, 0.0f, 0.2f);
                }
              }
            }
            else if (this.game.EditObj.ShowLISRange & !this.game.EditObj.AIMoving & !Information.IsNothing( this.game.EditObj.TempSup[0]))
            {
              if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[cx, cy].Regime))
              {
                let mut num142: i32 =  this.game.EditObj.TempSup[0].Value[cx, cy];
                let mut num143: i32 =  50;
                if (num142 <= num143 * 1)
                {
                  ref Graphics local259 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local260: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local259, ref local260, x, y, -1f, -0.0f, -1f, 0.1f);
                }
                else if ( num142 <=  num143 * 1.33)
                {
                  ref Graphics local261 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local262: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local261, ref local262, x, y, -0.0f, -0.0f, -1f, 0.1f);
                }
                else if ( num142 <=  num143 * 1.66)
                {
                  ref Graphics local263 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local264: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local263, ref local264, x, y, -1f, -0.0f, -0.0f, 0.1f);
                }
                else if (num142 <= num143 * 2)
                {
                  ref Graphics local265 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local266: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local265, ref local266, x, y, 0.0f, -1f, -1f, 0.1f);
                }
                else
                {
                  ref Graphics local267 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                  ref local268: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx;
                  let mut y: i32 =  ty;
                  DrawMod.Draw(ref local267, ref local268, x, y, -1f, -1f, -1f, 0.1f);
                }
              }
            }
            else if (this.game.EditObj.UnitSelected > -1 & this.game.EditObj.ShowHQPower & !this.game.EditObj.AIMoving && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ & this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime && this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[cx, cy].Regime))
            {
              let mut num144: i32 =  this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0, cx, cy, 0, (int) Math.Round( (this.game.Data.RuleVar[73] +  (int) Math.Round( (100f / this.game.Data.RuleVar[74])))));
              num145: i32;
              if ( num144 <=  this.game.Data.RuleVar[73])
              {
                num145 = 100;
              }
              else
              {
                num145 = (int) Math.Round(100.0 -  this.game.Data.RuleVar[74] * ( num144 -  this.game.Data.RuleVar[73]));
                if (0 > num145)
                  num145 = 0;
              }
              if (num145 >= 100)
              {
                ref Graphics local269 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                ref local270: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.Draw(ref local269, ref local270, x, y, -1f, -0.0f, -1f, 0.2f);
              }
              else if (num145 >= 75)
              {
                ref Graphics local271 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                ref local272: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.Draw(ref local271, ref local272, x, y, -0.0f, -0.0f, -1f, 0.2f);
              }
              else if (num145 >= 50)
              {
                ref Graphics local273 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                ref local274: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.Draw(ref local273, ref local274, x, y, -1f, -0.0f, -0.0f, 0.2f);
              }
              else if (num145 >= 1)
              {
                ref Graphics local275 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEX, Zoom);
                ref local276: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.Draw(ref local275, ref local276, x, y, 0.0f, -1f, -1f, 0.2f);
              }
            }
          }
          num146: i32;
          if (!this.game.EditObj.AIMoving & !InfoMode && this.game.EditObj.LayerSupplyOn | !this.game.EditObj.LayerSupplyOn & !this.game.EditObj.HideAS && !flag1 & !ispredrawing & this.game.Data.Round > 0 && this.game.EditObj.Zoom > -1)
          {
            if ( this.game.Data.MapObj[cmap].HexObj[cx, cy].get_ReconPts(index5) >=  this.game.Data.RuleVar[8] | !this.game.Data.FOWOn && this.game.Data.MapObj[cmap].HexObj[cx, cy].DammageVisible > 0 && this.game.Data.MapObj[cmap].HexObj[cx, cy].DammageVisible > 999)
              num146 = 999;
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SupplyLost(index5) > 0)
            {
              let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SupplyLost(index5);
              if (Number > 999)
                Number = 999;
              if (Zoom == 0)
              {
                ref Graphics local277 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.REDOVAL);
                ref local278: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 7;
                let mut y: i32 =  ty + 3;
                DrawMod.Draw(ref local277, ref local278, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 5, ty + 6, Color.White);
              }
              else
              {
                ref Graphics local279 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.REDOVAL);
                ref local280: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 23;
                let mut y: i32 =  ty + 3;
                DrawMod.Draw(ref local279, ref local280, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 23, ty + 6, Color.White);
              }
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_PowerPointsLost(index5) > 0)
            {
              let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_PowerPointsLost(index5);
              if (Number > 999)
                Number = 999;
              if (Zoom == 0)
              {
                ref Graphics local281 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.REDOVAL);
                ref local282: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 35;
                let mut y: i32 =  ty + 3;
                DrawMod.Draw(ref local281, ref local282, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 33, ty + 6, Color.White);
              }
              else
              {
                ref Graphics local283 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.REDOVAL);
                ref local284: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 82;
                let mut y: i32 =  ty + 3;
                DrawMod.Draw(ref local283, ref local284, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 80, ty + 6, Color.White);
              }
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SupplyKilled(index5) > 0)
            {
              let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SupplyKilled(index5);
              if (Number > 999)
                Number = 999;
              if (Zoom == 0)
              {
                ref Graphics local285 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.BLUEOVAL);
                ref local286: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 7;
                let mut y: i32 =  ty + 23;
                DrawMod.Draw(ref local285, ref local286, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 5, ty + 26, Color.White);
              }
              else
              {
                ref Graphics local287 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.BLUEOVAL);
                ref local288: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 23;
                let mut y: i32 =  ty + 70;
                DrawMod.Draw(ref local287, ref local288, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 21, ty + 73, Color.White);
              }
            }
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_PowerPointsKilled(index5) > 0)
            {
              let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_PowerPointsKilled(index5);
              if (Number > 999)
                Number = 999;
              if (Zoom == 0)
              {
                ref Graphics local289 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.BROWNOVAL);
                ref local290: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 35;
                let mut y: i32 =  ty + 23;
                DrawMod.Draw(ref local289, ref local290, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 33, ty + 24, Color.White);
              }
              else
              {
                ref Graphics local291 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.BROWNOVAL);
                ref local292: Bitmap = ref bitmap4;
                let mut x: i32 =  tx + 82;
                let mut y: i32 =  ty + 70;
                DrawMod.Draw(ref local291, ref local292, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new("Times New Roman", 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 80, ty + 71, Color.White);
              }
            }
            if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime, this.game.Data.Turn))
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattlePenalty(index5) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattlePenalty(index5) > 999)
                  num146 = 999;
                if (Zoom == 0)
                  ;
              }
            }
            else if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStack(index5) > 0)
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStack(index5) > 999)
                num146 = 999;
              if (Zoom == 0)
                ;
            }
          }
          if (this.game.Data.FOWOn & this.game.EditObj.RealRound > 0 & !InfoMode)
          {
            if (!flag1)
            {
              if (this.game.EditObj.PrefShowFOW)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].MaxRecon < 1)
                {
                  if (!NoShader)
                  {
                    ref Graphics local293 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, Zoom);
                    ref local294: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx;
                    let mut y: i32 =  ty;
                    DrawMod.DrawSimple(ref local293, ref local294, x, y);
                  }
                  num146 = 0;
                  let mut num147: i32 =  0;
                  index1 = 0;
                  do
                  {
                    coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index1 + 1);
                    if (!coordinate1.onmap)
                      numArray32[index1] = 0;
                    else if (this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].get_LastLT(this.game.Data.Turn) == -1)
                    {
                      numArray32[index1] = 1;
                      num147 = 1;
                    }
                    else
                      numArray32[index1] = 0;
                    index1 += 1;
                  }
                  while (index1 <= 5);
                  if (num147 == 1 & this.game.Data.ShrowdOn)
                  {
                    let mut shrowdsheet: i32 =  this.game.SHROWDSHEET;
                    let mut index112: i32 =  this.game.SPRITE64[numArray32[0], numArray32[1], numArray32[2], numArray32[3], numArray32[4], numArray32[5]];
                    ref Graphics local295 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(shrowdsheet, Zoom);
                    ref local296: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index112] * num6, this.game.SHEETY[index112] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local295, ref local296, srcrect, destrect);
                  }
                }
                else
                {
                  let mut num148: i32 =  0;
                  let mut num149: i32 =  0;
                  index1 = 0;
                  do
                  {
                    coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index1 + 1);
                    if (!coordinate1.onmap)
                    {
                      numArray31[index1] = 0;
                      numArray32[index1] = 0;
                    }
                    else
                    {
                      if (this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].MaxRecon < 1)
                      {
                        numArray31[index1] = 1;
                        num148 = 1;
                      }
                      else
                        numArray31[index1] = 0;
                      if (this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].get_LastLT(this.game.Data.Turn) == -1)
                      {
                        numArray32[index1] = 1;
                        num149 = 1;
                      }
                      else
                        numArray32[index1] = 0;
                    }
                    index1 += 1;
                  }
                  while (index1 <= 5);
                  if (num148 == 1 && !NoShader)
                  {
                    let mut fogsheet: i32 =  this.game.FOGSHEET;
                    let mut index113: i32 =  this.game.SPRITE64[numArray31[0], numArray31[1], numArray31[2], numArray31[3], numArray31[4], numArray31[5]];
                    ref Graphics local297 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(fogsheet, Zoom);
                    ref local298: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index113] * num6, this.game.SHEETY[index113] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local297, ref local298, srcrect, destrect);
                  }
                  if (num149 == 1 & this.game.Data.ShrowdOn)
                  {
                    let mut shrowdsheet: i32 =  this.game.SHROWDSHEET;
                    let mut index114: i32 =  this.game.SPRITE64[numArray32[0], numArray32[1], numArray32[2], numArray32[3], numArray32[4], numArray32[5]];
                    ref Graphics local299 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(shrowdsheet, Zoom);
                    ref local300: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index114] * num6, this.game.SHEETY[index114] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local299, ref local300, srcrect, destrect);
                  }
                }
              }
            }
            else if (flag1 && this.game.Data.ShrowdOn && this.game.Data.MapObj[0].HexObj[cx, cy].get_LastLT(index5) > -1)
            {
              let mut num150: i32 =  0;
              index1 = 0;
              do
              {
                coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index1 + 1);
                if (!coordinate1.onmap)
                  numArray32[index1] = 0;
                else if (this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].get_LastLT(index5) == -1)
                {
                  numArray32[index1] = 1;
                  num150 = 1;
                }
                else
                  numArray32[index1] = 0;
                index1 += 1;
              }
              while (index1 <= 5);
              if (num150 == 1 & this.game.Data.ShrowdOn)
              {
                let mut shrowdsheet: i32 =  this.game.SHROWDSHEET;
                let mut index115: i32 =  this.game.SPRITE64[numArray32[0], numArray32[1], numArray32[2], numArray32[3], numArray32[4], numArray32[5]];
                ref Graphics local301 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(shrowdsheet, Zoom);
                ref local302: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(this.game.SHEETX[index115] * num6, this.game.SHEETY[index115] * num7, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local301, ref local302, srcrect, destrect);
              }
            }
          }
          str: String;
          if (!ispredrawing & !InfoMode & this.game.EditObj.ShowLabel)
          {
            let mut num151: i32 =  0;
            if (index5 > -1)
            {
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].Regime == index5)
                num151 = 1;
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].get_ReconPts(this.game.EditObj.RealTurn) >= 1)
                num151 = 1;
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].MaxRecon >= 1 & !flag1 & index5 == this.game.EditObj.RealTurn)
                num151 = 1;
            }
            else
              num151 = 1;
            if (!this.game.Data.ShrowdOn)
              num151 = 1;
            if (num151 == 1)
            {
              if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].SmallLabel) > 0 & Zoom >= 0)
              {
                let mut num152: i32 =  (int) Math.Round(Math.Floor( this.game.Data.MapObj[cmap].HexObj[cx, cy].SmallLabelType / 10.0));
                let mut tcol: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].SmallLabelType - num152 * 10;
                strArray: Vec<String> = this.game.Data.MapObj[cmap].HexObj[cx, cy].SmallLabel.ToUpper().Split(' ');
                str = "";
                let mut upperBound: i32 =  strArray.GetUpperBound(0);
                for (let mut index116: i32 =  0; index116 <= upperBound; index116 += 1)
                {
                  if (Zoom < 1 & strArray[index116].Length > 9)
                    strArray[index116] = Strings.Left(strArray[index116], 9);
                  if (Zoom == 1 & strArray[index116].Length > 11)
                    strArray[index116] = Strings.Left(strArray[index116], 11);
                  if ((num152 == 1 | num152 == 2) & Zoom < 1 & strArray[index116].Length > 7)
                    strArray[index116] = Strings.Left(strArray[index116], 7);
                  if (index116 > 0)
                    str += " ";
                  str += strArray[index116];
                }
                switch (num152)
                {
                  case 0:
                    switch (Zoom)
                    {
                      case 0:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont11, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), tcol);
                        break;
                      case 1:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont7, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), tcol);
                        break;
                    }
                    break;
                  case 1:
                    switch (Zoom)
                    {
                      case 0:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont11, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 4.0), tcol);
                        break;
                      case 1:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont7, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 4.0), tcol);
                        break;
                    }
                    break;
                  case 2:
                    switch (Zoom)
                    {
                      case 0:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont11, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 * 0.75), tcol);
                        break;
                      case 1:
                        DrawMod.DrawTextCenterSmallLabelMultiLine(ref toG, str, this.game.MarcFont7, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 * 0.75), tcol);
                        break;
                    }
                    break;
                }
              }
              if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 <= 5)
                {
                  c: Color = Color.Transparent;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 1)
                    c = Color.White;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 2)
                    c = Color.FromArgb((int) byte.MaxValue, 150, 200, (int) byte.MaxValue);
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 3)
                    c = Color.Black;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 4)
                    c = Color.Red;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 5)
                    c = Color.Yellow;
                  if (Zoom == 0)
                  {
                    str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 6, c, true);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 6, c, true);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 27, ty + 6, c, true);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 37, ty + 6, c, true);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 48, ty + 6, c, true);
                  }
                  else if ( this.game.Data.RuleVar[352] == 0.0 & Zoom == 1)
                  {
                    index1 = 1;
                    do
                    {
                      str = Strings.UCase(Strings.Mid(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, index1, 1));
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredConsoleCenter(ref toG, str, this.game.MarcFont1, tx + 13 + 10 + 21 * (index1 - 1), ty + 13, Color.Black);
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredConsoleCenter(ref toG, str, this.game.MarcFont1, tx + 12 + 10 + 21 * (index1 - 1), ty + 12, c);
                      index1 += 1;
                    }
                    while (index1 <= 5);
                  }
                  else if (Zoom == -1)
                  {
                    str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 2, ty + 0, c);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 0, c);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 12, ty + 0, c);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 0, c);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 22, ty + 0, c);
                  }
                }
                else
                {
                  switch (Zoom)
                  {
                    case -1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 1));
                        SizeF sizeF2 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 7)
                          c = Color.FromArgb((int) byte.MaxValue, 150, 200, (int) byte.MaxValue);
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 16) - sizeF2.Width / 2f)), ty - 3, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 0:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 1));
                        SizeF sizeF3 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 7)
                          c = Color.FromArgb((int) byte.MaxValue, 150, 200, (int) byte.MaxValue);
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 32) - sizeF3.Width / 2f)), ty + 4, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 1));
                        SizeF sizeF4 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 32f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 7)
                          c = Color.FromArgb((int) byte.MaxValue, 150, 200, (int) byte.MaxValue);
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 32f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 64) - sizeF4.Width / 2f)), ty + 8, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                  }
                }
              }
              if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 <= 5)
                {
                  c: Color = Color.Transparent;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 1)
                    c = Color.White;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 2)
                    c = Color.FromArgb((int) byte.MaxValue, 150, 200, (int) byte.MaxValue);
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 3)
                    c = Color.Black;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 4)
                    c = Color.Red;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 5)
                    c = Color.Yellow;
                  if (Zoom == 0)
                  {
                    str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 30, c, true);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 30, c, true);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 27, ty + 30, c, true);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 37, ty + 30, c, true);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 48, ty + 30, c, true);
                  }
                  else if ( this.game.Data.RuleVar[352] == 0.0 & Zoom == 1)
                  {
                    index1 = 1;
                    do
                    {
                      str = Strings.UCase(Strings.Mid(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, index1, 1));
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredConsoleCenter(ref toG, str, this.game.MarcFont1, tx + 14 + 20 * (index1 - 1) + 13, ty + 13 + 48, Color.Black);
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredConsoleCenter(ref toG, str, this.game.MarcFont1, tx + 13 + 20 * (index1 - 1) + 13, ty + 12 + 48, c);
                      index1 += 1;
                    }
                    while (index1 <= 5);
                  }
                  else if (Zoom == -1)
                  {
                    str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 1, ty + 12, c);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 6, ty + 12, c);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 11, ty + 12, c);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 16, ty + 12, c);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 21, ty + 12, c);
                  }
                }
                else
                {
                  switch (Zoom)
                  {
                    case -1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 1));
                        SizeF sizeF5 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 16) - sizeF5.Width / 2f)), ty + 9, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 0:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 1));
                        SizeF sizeF6 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 32) - sizeF6.Width / 2f)), ty + 28, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
                      {
                        str = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 1));
                        SizeF sizeF7 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 32f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 32f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 64) - sizeF7.Width / 2f)), ty + 56, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                  }
                }
              }
            }
          }
          if (!this.game.EditObj.AIMoving & !InfoMode && this.game.EditObj.LayerSupplyOn | !this.game.EditObj.LayerSupplyOn & !this.game.EditObj.HideAS && !flag1 & !ispredrawing & this.game.EditObj.RealRound > 0 & this.game.EditObj.Zoom > -1)
          {
            if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime, this.game.Data.Turn))
            {
              let mut num153: i32 =  0;
              let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
              for (let mut index117: i32 =  0; index117 <= regimeCounter; index117 += 1)
              {
                let mut num154: i32 =  0;
                if (this.game.HandyFunctionsObj.IsHostileNotSelf(this.game.Data.Turn, index117) && this.game.Data.MapObj[0].HexObj[cx, cy].get_ZocPts(index117) > 0 && this.game.HandyFunctionsObj.VisibleEnemyUnitsInOrAroundHEx(cx, cy, 0, this.game.Data.Turn))
                  num154 = (int) Math.Round( ( num154 + this.game.Data.RuleVar[323]));
                if (num154 > num153)
                  num153 = num154;
              }
              if (num153 > 0 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattlePenalty(index5) > 0 | this.game.Data.MapObj[cmap].HexObj[cx, cy].get_APPenalty(index5) > 0)
              {
                let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattlePenalty(index5) + this.game.Data.MapObj[cmap].HexObj[cx, cy].get_APPenalty(index5) + num153;
                if (Number > 999)
                  Number = 999;
                if (Zoom == 0)
                {
                  ref Graphics local303 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKBOX);
                  ref local304: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 22;
                  let mut y: i32 =  ty + 23;
                  DrawMod.Draw(ref local303, ref local304, x, y, 0.0f, 0.0f, 0.0f, 0.3f);
                  DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 20, ty + 26, Color.FromArgb(180, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                }
                else
                {
                  ref Graphics local305 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKBOX);
                  ref local306: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 54;
                  let mut y: i32 =  ty + 70;
                  DrawMod.Draw(ref local305, ref local306, x, y, 0.0f, 0.0f, 0.0f, 0.3f);
                  DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 51, ty + 73, Color.FromArgb(180, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue));
                }
              }
            }
            else
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStackArt(index5) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStackArt(index5) > 999)
                  num146 = 999;
                if (Zoom == 0)
                {
                  ref Graphics local307 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local308: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 9;
                  let mut y: i32 =  ty + 23;
                  DrawMod.Draw(ref local307, ref local308, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, "art", Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 11, ty + 26, Color.White);
                }
                else
                {
                  ref Graphics local309 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local310: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 26;
                  let mut y: i32 =  ty + 70;
                  DrawMod.Draw(ref local309, ref local310, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, "art", Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 28, ty + 73, Color.White);
                }
              }
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStackAir(index5) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStackAir(index5) > 999)
                  num146 = 999;
                if (Zoom == 0)
                {
                  ref Graphics local311 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local312: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 29;
                  let mut y: i32 =  ty + 23;
                  DrawMod.Draw(ref local311, ref local312, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, "air", Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 31, ty + 26, Color.White);
                }
                else
                {
                  ref Graphics local313 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local314: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 82;
                  let mut y: i32 =  ty + 70;
                  DrawMod.Draw(ref local313, ref local314, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, "air", Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 84, ty + 73, Color.White);
                }
              }
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStack(index5) > 0)
              {
                let mut Number: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_BattleStack(index5);
                if (Number > 999)
                  Number = 999;
                if (Zoom == 0)
                {
                  ref Graphics local315 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local316: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 22;
                  let mut y: i32 =  ty + 23;
                  DrawMod.Draw(ref local315, ref local316, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 21, ty + 26, Color.White);
                }
                else
                {
                  ref Graphics local317 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.BLACKOVAL);
                  ref local318: Bitmap = ref bitmap4;
                  let mut x: i32 =  tx + 54;
                  let mut y: i32 =  ty + 70;
                  DrawMod.Draw(ref local317, ref local318, x, y, 0.0f, 0.0f, 0.0f, 0.5f);
                  DrawMod.DrawTextColouredMarc(ref toG, Conversion.Str( Number), Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 52, ty + 73, Color.White);
                }
              }
            }
          }
          num155: i32;
          if (!InfoMode &&  this.game.Data.RuleVar[403] > 0.0 & !this.game.EditObj.AIMoving)
          {
            let mut num156: i32 =  0;
            if (this.game.EditObj.layerLisAvailable)
              num156 = 1;
            if (this.game.EditObj.layerLisUsed)
              num156 = 2;
            if (this.game.EditObj.layerLisTotal)
              num156 = 3;
            if (this.game.EditObj.layerLisBottlenecks)
              num156 = 4;
            if (this.game.EditObj.layerLisPreview)
              num156 = 5;
            if (cx == 54 & cy == 36)
              cx = cx;
            if (num156 > 0)
            {
              let mut val1_1: i32 =  0;
              let mut val1_2: i32 =  0;
              let mut index118: i32 =  0;
              do
              {
                let mut widdy1: i32 =  0;
                let mut num157: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[index118];
                num16 = 0;
                let mut widdy2: i32 =  0;
                index9 = 0;
                index10 = 0;
                let mut widdy3: i32 =  0;
                switch (num156)
                {
                  case 1:
                    let mut liSpoint: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LISpoints[index118];
                    if (index118 < 6)
                    {
                      if (this.game.Data.MapObj[0].HexObj[cx, cy].LISpoints[6] < liSpoint)
                        liSpoint = this.game.Data.MapObj[0].HexObj[cx, cy].LISpoints[6];
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                      if (coordinate1.onmap && this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6] < liSpoint)
                        liSpoint = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6];
                    }
                    if (num157 > 0)
                      val1_1 = Math.Max(val1_1, liSpoint);
                    if (index118 == 6 & liSpoint >= 0)
                      val1_1 = Math.Min(val1_1, liSpoint);
                    widdy1 = (int) Math.Round(Math.Sqrt(Math.Sqrt( liSpoint)));
                    break;
                  case 2:
                    let mut num158: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIShistory[index118];
                    if (num157 > 0)
                      val1_1 = Math.Max(val1_1, num158);
                    widdy1 = (int) Math.Round(Math.Sqrt(Math.Sqrt( num158)));
                    let mut d1: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LISorganic[index118];
                    index10 = d1;
                    index9 = this.game.Data.MapObj[0].HexObj[cx, cy].LISorganicPercentage[index118];
                    widdy2 = (int) Math.Round(Math.Sqrt(Math.Sqrt( d1)));
                    break;
                  case 3:
                    let mut num159: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[index118];
                    if (num157 > 0)
                      val1_1 = Math.Max(val1_1, num159);
                    widdy1 = (int) Math.Round(Math.Sqrt(Math.Sqrt( num159)));
                    break;
                  case 4:
                    let mut num160: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[index118] - this.game.Data.MapObj[0].HexObj[cx, cy].LIShistory[index118];
                    if (index118 < 6)
                    {
                      let mut num161: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[cx, cy].LIShistory[6];
                      if (num161 < num160)
                        num160 = num161;
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                      if (coordinate1.onmap)
                      {
                        let mut num162: i32 =  this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[6];
                        if (num162 < num160)
                          num160 = num162;
                      }
                    }
                    num16 = num157 <= 0 ? 0 : (int) Math.Round( (100 * num160) /  num157);
                    let mut num163: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[index118];
                    if (num157 > 0)
                      val1_1 = Math.Max(val1_1, num163);
                    widdy1 = (int) Math.Round(Math.Sqrt(Math.Sqrt( num163)));
                    let mut d2: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].LISorganic[index118];
                    index9 = this.game.Data.MapObj[0].HexObj[cx, cy].LISorganicPercentage[6];
                    widdy2 = (int) Math.Round(Math.Sqrt(Math.Sqrt( d2)));
                    break;
                  case 5:
                    let mut val1_3: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewLIS[index118];
                    let mut val2: i32 =  val1_3;
                    if (index118 < 6)
                    {
                      if (this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewLIS[6] < val1_3)
                        val1_3 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewLIS[6];
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                      if (coordinate1.onmap)
                      {
                        if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[6] < val1_3)
                          val1_3 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[6];
                        let mut index119: i32 =  index118 + 3;
                        if (index119 > 5)
                          index119 -= 6;
                        val2 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[index119];
                      }
                    }
                    let mut num164: i32 =  Math.Min(val1_3, val2);
                    let mut num165: i32 =  num164;
                    if (num164 > 0)
                      val1_1 = Math.Max(val1_1, num165);
                    if (index118 == 6 & num165 >= 0)
                      val1_1 = Math.Min(val1_1, num165);
                    widdy1 = (int) Math.Round(Math.Sqrt(Math.Sqrt( num165)));
                    if (this.game.EditObj.layerLisOnlyAssetId > 0)
                    {
                      let mut num166: i32 =  this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewAssetLIS[index118];
                      if (index118 < 6 & !this.game.EditObj.layerLisOnlyAssetId_isSupplyBase)
                      {
                        if (this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewAssetLIS[6] < num166)
                          num166 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewAssetLIS[6];
                        coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                        if (coordinate1.onmap && this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewAssetLIS[6] < num166)
                          num166 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewAssetLIS[6];
                      }
                      if (num166 > 0)
                        val1_2 = Math.Max(val1_2, num166);
                      if (index118 == 6 & num166 >= 0)
                        val1_2 = Math.Min(val1_2, num166);
                      widdy3 = (int) Math.Round(Math.Sqrt(Math.Sqrt( num166)));
                      break;
                    }
                    break;
                }
                num167: i32;
                num168: i32;
                if (widdy1 > 0 | widdy2 > 0)
                {
                  if (this.game.EditObj.Zoom == 1)
                  {
                    widdy1 *= 2;
                    widdy2 *= 2;
                  }
                  if (this.game.EditObj.Zoom == -1)
                  {
                    widdy1 = (int) Math.Round(Math.Max(1.0,  widdy1 / 2.0));
                    widdy2 = (int) Math.Round(Math.Max(1.0,  widdy2 / 2.0));
                  }
                  num169: i32;
                  num170: i32;
                  num171: i32;
                  if ((num156 == 2 | num156 == 4) & widdy2 > 0)
                  {
                    if (index9 >= 100)
                    {
                      num169 = 0;
                      num170 = (int) byte.MaxValue;
                      num171 = 0;
                    }
                    else if (index9 >= 76)
                    {
                      num169 = 225;
                      num170 = 225;
                      num171 = 0;
                    }
                    else if (index9 >= 51)
                    {
                      num169 = 0;
                      num170 = 200;
                      num171 = (int) byte.MaxValue;
                    }
                    else if (index9 >= 26)
                    {
                      num169 = (int) byte.MaxValue;
                      num170 = 0;
                      num171 = 0;
                    }
                    else
                    {
                      num169 = 0;
                      num170 = 0;
                      num171 = 0;
                    }
                  }
                  if (num156 == 4)
                  {
                    if (num16 >= 76)
                    {
                      a3 = 0.0f;
                      a4 =  byte.MaxValue;
                      a5 = 0.0f;
                    }
                    if (num16 < 76 & num16 >= 51)
                    {
                      a3 = 225f;
                      a4 = 225f;
                      a5 = 0.0f;
                    }
                    if (num16 <= 50 & num16 >= 26)
                    {
                      a3 = 0.0f;
                      a4 = 200f;
                      a5 =  byte.MaxValue;
                    }
                    if (num16 <= 25 & num16 >= 1)
                    {
                      a3 =  byte.MaxValue;
                      a4 = 0.0f;
                      a5 = 0.0f;
                    }
                    if (num16 <= 0)
                    {
                      a3 = 0.0f;
                      a4 = 0.0f;
                      a5 = 0.0f;
                    }
                    widdy2 = 2;
                  }
                  if (num156 == 1)
                  {
                    a3 = 0.0f;
                    a4 = 200f;
                    a5 = 0.0f;
                    if (index118 < 6 && val1_1 < this.game.Data.MapObj[0].HexObj[cx, cy].LISpoints[6])
                    {
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                      if (val1_1 < this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6])
                      {
                        a3 = 180f;
                        a4 = 180f;
                        a5 = 0.0f;
                        if (Zoom == 1)
                        {
                          if (index118 == 0)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 2.0), ty + 14, 4);
                          if (index118 == 1)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( (num6 * 2) / 3.0), ty + 24, 4);
                          if (index118 == 2)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( (num6 * 3) / 4.0) + 4, ty + 50, 4);
                          if (index118 == 3)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 2.0), ty + 82, 4);
                          if (index118 == 4)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 4.0) - 4, ty + 50, 4);
                          if (index118 == 5)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 3.0), ty + 24, 4);
                        }
                      }
                    }
                  }
                  if (num156 == 3)
                  {
                    a3 = 50f;
                    a4 = 50f;
                    a5 = 50f;
                    if (index118 < 6 && val1_1 < this.game.Data.MapObj[0].HexObj[cx, cy].LIStotalHistory[6])
                    {
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, index118 + 1);
                      if (val1_1 < this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[6])
                      {
                        a3 = 100f;
                        a4 = 100f;
                        a5 = 0.0f;
                        if (Zoom == 1)
                        {
                          if (index118 == 0)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 2.0), ty + 14, 4);
                          if (index118 == 1)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( (num6 * 2) / 3.0), ty + 24, 4);
                          if (index118 == 2)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( (num6 * 3) / 4.0) + 4, ty + 50, 4);
                          if (index118 == 3)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 2.0), ty + 82, 4);
                          if (index118 == 4)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 4.0) - 4, ty + 50, 4);
                          if (index118 == 5)
                            DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, tx + (int) Math.Round( num6 / 3.0), ty + 24, 4);
                        }
                      }
                    }
                  }
                  if (index118 == 6)
                  {
                    num167 = (int) Math.Round( tx +  num6 / 2.0);
                    num168 = (int) Math.Round( ty +  num7 / 2.0);
                    if ((num156 == 2 | num156 == 4) & widdy2 > 0)
                      DrawMod.DrawBlock(ref toG, num167 - widdy2, num168 - widdy2, widdy2 * 2, widdy2 * 2, num169, num170, num171, (int) byte.MaxValue);
                    if (widdy1 > 0)
                    {
                      if (num156 == 5)
                        DrawMod.DrawBlock(ref toG, num167 - widdy1, num168 - widdy1, widdy1 * 2, widdy1 * 2, 140, 100, 140, 160);
                      if (num156 == 3)
                        DrawMod.DrawBlock(ref toG, num167 - widdy1, num168 - widdy1, widdy1 * 2, widdy1 * 2, (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), 100);
                      if (num156 == 1)
                        DrawMod.DrawBlock(ref toG, num167 - widdy1, num168 - widdy1, widdy1 * 2, widdy1 * 2, (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), 155);
                      if (num156 == 2)
                        DrawMod.DrawBlock(ref toG, num167 - widdy1, num168 - widdy1, widdy1 * 2, widdy1 * 2, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 155);
                      if (num156 == 4)
                        DrawMod.DrawBlock(ref toG, num167 - widdy1, num168 - widdy1, widdy1 * 2, widdy1 * 2, (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), (int) byte.MaxValue);
                    }
                  }
                  else if (index118 < 6)
                  {
                    if (index118 == 0)
                    {
                      num167 = 0;
                      num168 = (int) Math.Round(-( num7 / 2.0));
                    }
                    if (index118 == 1)
                    {
                      num167 = (int) Math.Round( num6 * 0.4);
                      num168 = (int) Math.Round(-( num7 * 0.25));
                    }
                    if (index118 == 2)
                    {
                      num167 = (int) Math.Round( num6 * 0.4);
                      num168 = (int) Math.Round( num7 * 0.25);
                    }
                    if (index118 == 3)
                    {
                      num167 = 0;
                      num168 = (int) Math.Round( num7 / 2.0);
                    }
                    if (index118 == 4)
                    {
                      num167 = (int) Math.Round(-( num6 * 0.4));
                      num168 = (int) Math.Round( num7 * 0.25);
                    }
                    if (index118 == 5)
                    {
                      num167 = (int) Math.Round(-( num6 * 0.4));
                      num168 = (int) Math.Round(-( num7 * 0.25));
                    }
                    if ((num156 == 2 | num156 == 4) & widdy2 > 0)
                      DrawMod.drawLineDot(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), Color.FromArgb((int) byte.MaxValue, num169, num170, num171), widdy2);
                    if (widdy1 > 0)
                    {
                      if (num156 == 5)
                        DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), 140, 100, 140, 160, widdy1);
                      if (num156 == 3)
                        DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), 100, widdy1);
                      if (num156 == 1)
                        DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), 155, widdy1);
                      if (num156 == 2)
                        DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, 155, widdy1);
                      if (num156 == 4)
                        DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), (int) Math.Round( a3), (int) Math.Round( a4), (int) Math.Round( a5), (int) byte.MaxValue, widdy1);
                    }
                  }
                }
                if (widdy3 > 0 & num156 == 5)
                {
                  if (this.game.EditObj.Zoom == 1)
                    widdy3 *= 2;
                  if (this.game.EditObj.Zoom == -1)
                    widdy3 = (int) Math.Round(Math.Max(1.0,  widdy3 / 2.0));
                  if (index118 == 6)
                  {
                    num167 = (int) Math.Round( tx +  num6 / 2.0);
                    num168 = (int) Math.Round( ty +  num7 / 2.0);
                    if (widdy3 > 0 && num156 == 5)
                      DrawMod.DrawBlock(ref toG, num167 - widdy3, num168 - widdy3, widdy3 * 2, widdy3 * 2, 225, 130, 225, (int) byte.MaxValue);
                  }
                  else if (index118 < 6)
                  {
                    if (index118 == 0)
                    {
                      num167 = 0;
                      num168 = (int) Math.Round(-( num7 / 2.0));
                    }
                    if (index118 == 1)
                    {
                      num167 = (int) Math.Round( num6 * 0.4);
                      num168 = (int) Math.Round(-( num7 * 0.25));
                    }
                    if (index118 == 2)
                    {
                      num167 = (int) Math.Round( num6 * 0.4);
                      num168 = (int) Math.Round( num7 * 0.25);
                    }
                    if (index118 == 3)
                    {
                      num167 = 0;
                      num168 = (int) Math.Round( num7 / 2.0);
                    }
                    if (index118 == 4)
                    {
                      num167 = (int) Math.Round(-( num6 * 0.4));
                      num168 = (int) Math.Round( num7 * 0.25);
                    }
                    if (index118 == 5)
                    {
                      num167 = (int) Math.Round(-( num6 * 0.4));
                      num168 = (int) Math.Round(-( num7 * 0.25));
                    }
                    if (widdy3 > 0 && num156 == 5)
                      DrawMod.drawLine(ref toG, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0), (int) Math.Round( tx +  num6 / 2.0 +  num167), (int) Math.Round( ty +  num7 / 2.0 +  num168), 225, 130, 225, (int) byte.MaxValue, widdy3);
                  }
                }
                index118 += 1;
              }
              while (index118 <= 6);
              if (Zoom == 0)
              {
                if (num156 == 4 & index9 > 0 & val1_1 < 1)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, index9.ToString() + "%", this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 4 & index9 > 0 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, "(" + index9.ToString() + "%)", this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 16.0), 11);
                if (num156 == 2 & index10 > 0 & val1_1 < 1)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, index10.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 2 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 1 & val1_1 > 0 & val1_1 < 9999999)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 2);
                if (num156 == 3 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 4 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, num16.ToString() + "%", this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 5 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 13);
                if (num156 == 5 & val1_2 > 0)
                {
                  if (this.game.EditObj.layerLisOnlyAssetId_isSupplyBase)
                    DrawMod.DrawTextCenterSmallLabel(ref toG, "+" + val1_2.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 2.0), 12);
                  else
                    DrawMod.DrawTextCenterSmallLabel(ref toG, val1_2.ToString(), this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 2.0), 12);
                }
              }
              if (Zoom == 1)
              {
                if (num156 == 4 & index9 > 0 & val1_1 < 1)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, index9.ToString() + "%", this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 4 & index9 > 0 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, "(" + index9.ToString() + "%)", this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 16.0), 11);
                if (num156 == 2 & index10 > 0 & val1_1 < 1)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, index10.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 99);
                if (num156 == 2 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 99);
                if (num156 == 1 & val1_1 > 0 & val1_1 < 9999999)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 2);
                if (num156 == 3 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 4 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, num16.ToString() + "%", this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 11);
                if (num156 == 5 & val1_1 > 0)
                  DrawMod.DrawTextCenterSmallLabel(ref toG, val1_1.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 + 16.0), 13);
                if (num156 == 5 & val1_2 > 0)
                {
                  if (this.game.EditObj.layerLisOnlyAssetId_isSupplyBase)
                    DrawMod.DrawTextCenterSmallLabel(ref toG, "+" + val1_2.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 2.0), 12);
                  else
                    DrawMod.DrawTextCenterSmallLabel(ref toG, val1_2.ToString(), this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 2.0), 12);
                }
              }
            }
            if (!this.game.EditObj.AIMoving && num156 > 0 | this.game.EditObj.udsUnitOrderMode == 53)
            {
              NeighboursExtra lisTraffic = this.game.HandyFunctionsObj.GetLisTraffic(cx, cy);
              let mut index120: i32 =  0;
              do
              {
                if (lisTraffic.data[index120] > 0)
                {
                  color: Color;
                  if (lisTraffic.data[index120] == 1)
                    color = Color.FromArgb((int) byte.MaxValue, 125, (int) byte.MaxValue, 125);
                  if (lisTraffic.data[index120] == 2)
                    color = Color.FromArgb((int) byte.MaxValue, 0, (int) byte.MaxValue, 0);
                  if (lisTraffic.data[index120] == 3)
                    color = Color.FromArgb((int) byte.MaxValue, 64, 152, 0);
                  if (lisTraffic.data[index120] == 4)
                    color = Color.FromArgb((int) byte.MaxValue, 152, 152, 0);
                  if (lisTraffic.data[index120] == 5)
                    color = Color.FromArgb((int) byte.MaxValue, 192, 128, 0);
                  if (lisTraffic.data[index120] == 6)
                    color = Color.FromArgb((int) byte.MaxValue, 192, 64, 0);
                  if (lisTraffic.data[index120] == 7)
                    color = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 0, 100);
                  if (index120 == 0)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 2.0 -  num6 / 8.0), (int) Math.Round( ty +  num7 / 8.0), (int) Math.Round( num6 / 4.0), (int) Math.Round( num7 / 16.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 1)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( (tx + num6) -  num6 / 4.0 -  num6 / 16.0), (int) Math.Round( ty +  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 2)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( (tx + num6) -  num6 / 4.0 -  num6 / 16.0), (int) Math.Round( (ty + num7) -  num7 / 4.0 -  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 3)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 2.0 -  num6 / 8.0), (int) Math.Round( (ty + num7) -  num7 / 8.0 -  num7 / 16.0), (int) Math.Round( num6 / 4.0), (int) Math.Round( num7 / 16.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 4)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( (ty + num7) -  num7 / 4.0 -  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 5)
                    DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  color = Color.FromArgb(200, (int) Math.Round( color.R / 2.0), (int) Math.Round( color.G / 2.0), (int) Math.Round( color.B / 2.0));
                  if (index120 == 0)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 2.0 -  num6 / 8.0), (int) Math.Round( ty +  num7 / 8.0), (int) Math.Round( num6 / 4.0), (int) Math.Round( num7 / 16.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 1)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( (tx + num6) -  num6 / 4.0 -  num6 / 16.0), (int) Math.Round( ty +  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 2)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( (tx + num6) -  num6 / 4.0 -  num6 / 16.0), (int) Math.Round( (ty + num7) -  num7 / 4.0 -  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 3)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 2.0 -  num6 / 8.0), (int) Math.Round( (ty + num7) -  num7 / 8.0 -  num7 / 16.0), (int) Math.Round( num6 / 4.0), (int) Math.Round( num7 / 16.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 4)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( (ty + num7) -  num7 / 4.0 -  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                  if (index120 == 5)
                    DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 4.0), (int) Math.Round( num6 / 16.0), (int) Math.Round( num7 / 4.0), (int) color.R, (int) color.G, (int) color.B, (int) byte.MaxValue);
                }
                index120 += 1;
              }
              while (index120 <= 5);
            }
            if (!this.game.EditObj.AIMoving && num156 > 0 | this.game.EditObj.udsUnitOrderMode == 53 && (int) Math.Round(Conversion.Val(this.game.Data.Designer)) >= 42 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & !this.game.Data.RegimeObj[this.game.Data.Turn].minimumDataUsage)
            {
              let mut num172: i32 =  0;
              let mut num173: i32 =  0;
              num16 = 0;
              let mut num174: i32 =  0;
              index9 = 0;
              bool flag24 = false;
              bool flag25 = false;
              bool flag26 = false;
              if (this.game.EditObj.layerLis_TraficWindowOpen == 1 & this.game.EditObj.layerLisPreview_LogMode == 0)
              {
                num172 = this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0].Value[cx, cy];
                flag25 = true;
              }
              else if (num156 == 3 | num156 == 2)
              {
                num172 = this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[0];
                num173 = this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[1];
                num16 = this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[2];
                num174 = this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[3];
                index9 = this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[4];
                if (this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[4] < -1)
                  index9 = Math.Abs(this.game.Data.MapObj[0].HexObj[cx, cy].LISpull[4]);
                flag24 = true;
              }
              else if (num156 == 5 | this.game.EditObj.layerLis_TraficWindowOpen == 1 & this.game.EditObj.layerLisPreview_LogMode == 1)
              {
                num172 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[0];
                num173 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[1];
                num16 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[2];
                num174 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[3];
                index9 = this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[4];
                if (this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[4] < -1)
                  index9 = Math.Abs(this.game.Data.MapObj[0].HexObj[cx, cy].tempPreviewPull[4]);
                flag26 = true;
              }
              else if (num156 == 1)
              {
                num172 = this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0].Value[cx, cy];
                flag25 = true;
              }
              switch (num172)
              {
                case -1:
                  str = "BLOCK";
                  break;
                case 0:
                  goto label_1808;
                default:
                  if (num172 > 0 & num172 < 1000)
                  {
                    str = num172.ToString();
                    break;
                  }
                  if (num172 >= 1000)
                  {
                    num155 = (int) Math.Round( num172 / 1000.0);
                    str = num155.ToString() + "K";
                    break;
                  }
                  if (num172 < -1 & num172 > -1000)
                  {
                    str = Math.Abs(num172).ToString() + "*";
                    break;
                  }
                  if (num172 <= -1000)
                  {
                    str = ((int) Math.Round( Math.Abs(num172) / 1000.0)).ToString() + "K" + "*";
                    break;
                  }
                  break;
              }
              color: Color;
              if (flag24)
                color = Color.Gray;
              else if (flag25)
                color = Color.FromArgb((int) byte.MaxValue, 100, 200, 100);
              else if (flag26)
                color = Color.FromArgb((int) byte.MaxValue, 180, 140, 180);
              if (Zoom == 0)
              {
                if (num173 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 0.0), (int) Math.Round( ty +  num7 / 2.0 - 21.0), (int) Math.Round( num6 / 8.0), 2, (int) byte.MaxValue, 0, 0, (int) byte.MaxValue);
                if (num16 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 1.0), (int) Math.Round( ty +  num7 / 2.0 - 21.0), (int) Math.Round( num6 / 8.0), 2, 0, (int) byte.MaxValue, 0, (int) byte.MaxValue);
                if (num174 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 2.0), (int) Math.Round( ty +  num7 / 2.0 - 21.0), (int) Math.Round( num6 / 8.0), 2, 0, 0, (int) byte.MaxValue, (int) byte.MaxValue);
                if (index9 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 3.0), (int) Math.Round( ty +  num7 / 2.0 - 21.0), (int) Math.Round( num6 / 8.0), 2, 0, 0, 0, (int) byte.MaxValue);
                DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 2.0 - 19.0), (int) Math.Round( num6 / 2.0), 11, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 2.0 - 19.0), (int) Math.Round( num6 / 2.0), 11, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                DrawMod.DrawTextCenterSmallLabel(ref toG, str, this.game.MarcFont13, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 14.0));
              }
              if (Zoom == 1)
              {
                if (num173 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 0.0), (int) Math.Round( ty +  num7 / 2.0 - 39.0), (int) Math.Round( num6 / 8.0), 4, (int) byte.MaxValue, 0, 0, (int) byte.MaxValue);
                if (num16 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 1.0), (int) Math.Round( ty +  num7 / 2.0 - 39.0), (int) Math.Round( num6 / 8.0), 4, 0, (int) byte.MaxValue, 0, (int) byte.MaxValue);
                if (num174 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 2.0), (int) Math.Round( ty +  num7 / 2.0 - 39.0), (int) Math.Round( num6 / 8.0), 4, 0, 0, (int) byte.MaxValue, (int) byte.MaxValue);
                if (index9 > 0)
                  DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0 +  num6 / 8.0 * 3.0), (int) Math.Round( ty +  num7 / 2.0 - 39.0), (int) Math.Round( num6 / 8.0), 4, 0, 0, 0, (int) byte.MaxValue);
                DrawMod.DrawBlock(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 2.0 - 35.0), (int) Math.Round( num6 / 2.0), 21, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                DrawMod.DrawRectangle(ref toG, (int) Math.Round( tx +  num6 / 4.0), (int) Math.Round( ty +  num7 / 2.0 - 35.0), (int) Math.Round( num6 / 2.0), 21, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
                DrawMod.DrawTextCenterSmallLabel(ref toG, str, this.game.MarcFont4, (int) Math.Round( tx +  num6 / 2.0), (int) Math.Round( ty +  num7 / 2.0 - 24.0));
              }
            }
          }
label_1808:
          if (this.game.EventRelatedObj.Helper_AirEnabled() & !flag1 && this.game.EditObj.Zoom >= 0)
          {
            this.slotAir = this.strId534slot;
            if (this.slotAir > -1)
            {
              let mut length: i32 =  this.game.Data.StringListObj[this.slotAir].Length;
              SimpleList Expression;
              for (let mut tdata2: i32 =  0; tdata2 <= length; tdata2 += 1)
              {
                if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 0])) == this.game.Data.RegimeObj[this.game.Data.Turn].id)
                {
                  if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 1])) == cx && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 2])) == cy && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 9])) >= 0)
                  {
                    if (Information.IsNothing( Expression))
                      Expression = SimpleList::new();
                    Expression.Add((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 8])), (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 6])), (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 9])), tdata2);
                  }
                  if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 3])) == cx && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 4])) == cy && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 9])) >= 0)
                  {
                    if (Information.IsNothing( Expression))
                      Expression = SimpleList::new();
                    Expression.Add((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 8])), (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 6])), (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAir].Data[tdata2, 9])), tdata2);
                  }
                }
              }
              if (!Information.IsNothing( Expression))
              {
                Expression.ReverseSortHighSpeed();
                let mut num175: i32 =  0;
                let mut counter: i32 =  Expression.Counter;
                for (let mut index121: i32 =  0; index121 <= counter; index121 += 1)
                {
                  tstring: String = this.game.HandyFunctionsObj.CovertNumberToLetter(Expression.Id[index121]);
                  color: Color = this.game.HandyFunctionsObj.Air_GetColor(Expression.Data2[index121]);
                  let mut tcol: i32 =  0;
                  if (index121 == 3 & Expression.Counter > 3)
                  {
                    num155 = Expression.Counter - 2;
                    tstring = "+" + num155.ToString();
                    color = Color.FromArgb(150, 0, 0, 0);
                  }
                  if (this.game.EditObj.Zoom == 1)
                  {
                    num175 += 22;
                    DrawMod.DrawBlock(ref toG, tx + num175, ty + 3, 18, 14, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                    DrawMod.DrawTextCenterSmallLabel(ref toG, tstring, this.game.MarcFont4, tx - 1 + num175 + 10, ty + 11, tcol);
                  }
                  else
                  {
                    num175 += 11;
                    DrawMod.DrawBlock(ref toG, tx + num175, ty + 1, 11, 10, (int) color.R, (int) color.G, (int) color.B, (int) color.A);
                    DrawMod.DrawTextCenterSmallLabel(ref toG, tstring, this.game.MarcFont13, tx + num175 + 5, ty + 6, tcol);
                  }
                  if (index121 >= 3)
                    break;
                }
              }
            }
          }
          if (!flag1 & !InfoMode)
          {
            if (this.game.EditObj.HideUnit > 0 & this.game.EditObj.layerUnits && this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitCounter > -1 & !InfoMode)
            {
              let mut unitCounter: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitCounter;
              let mut num176: i32 =  -1;
              num16 = -1;
              let mut num177: i32 =  0;
              index9 = -1;
              for (let mut index122: i32 =  unitCounter; index122 >= 0; index122 += -1)
              {
                if (this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index122], index5) > 0)
                  num16 += 1;
              }
              if (Zoom == 0 | !this.game.EditObj.SpreadUnit)
              {
                if (num16 > 3)
                {
                  num177 = num16 - 3;
                  num16 = 3;
                }
              }
              else if (Zoom == 1 && num16 > 13)
              {
                num177 = num16 - 13;
                num16 = 13;
              }
              if (0 > num177)
                num177 = 0;
              for (index1 = unitCounter; index1 >= 0; index1 += -1)
              {
                if (this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], index5) > 0)
                {
                  num176 += 1;
                  if (num176 >= num177)
                  {
                    index9 += 1;
                    bool forcehighlight = false;
                    if (this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1] == this.game.EditObj.UnitSelected)
                      forcehighlight = true;
                    switch (Zoom)
                    {
                      case -1:
                        this.DrawUnitSmall(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 0 + 7 - (int) Math.Round( num16 * 0.5) + index9 * 1, ty + 0 + 3 - (int) Math.Round( num16 * 0.5) + index9 * 1, true);
                        continue;
                      case 0:
                        this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx - 1 + 14 - num16 + index9 * 2, ty + 1 + 4 - num16 + index9 * 2, true, mostlyHidden: (index1 > 0));
                        continue;
                      default:
                        if (this.game.EditObj.SpreadUnit)
                        {
                          if (num177 == 0 & num16 == 0)
                          {
                            this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 30, true);
                            continue;
                          }
                          if (num177 == 0 & num16 == 1)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 1)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 2)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 2)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 3)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 4)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 4)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 30, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 5)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 6)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 30, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 7)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 19, true);
                            if (index9 == 7)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 40, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 8)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 7)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 30, true);
                            if (index9 == 8)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 51, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 9)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 7)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 51, true);
                            if (index9 == 8)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 19, true);
                            if (index9 == 9)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 19, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 10)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 7)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 51, true);
                            if (index9 == 8)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 19, true);
                            if (index9 == 9)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 19, true);
                            if (index9 == 10)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 40, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 11)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 7)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 51, true);
                            if (index9 == 8)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 19, true);
                            if (index9 == 9)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 19, true);
                            if (index9 == 10)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 40, true);
                            if (index9 == 11)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 40, true);
                              continue;
                            }
                            continue;
                          }
                          if (num177 == 0 & num16 == 12)
                          {
                            if (index9 == 0)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 8, true);
                            if (index9 == 1)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 30, true);
                            if (index9 == 2)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 24, ty + 51, true);
                            if (index9 == 3)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 8, true);
                            if (index9 == 4)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 30, true);
                            if (index9 == 5)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 67, ty + 51, true);
                            if (index9 == 6)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 8, true);
                            if (index9 == 7)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 51, true);
                            if (index9 == 8)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 19, true);
                            if (index9 == 9)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 19, true);
                            if (index9 == 10)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 35, ty + 40, true);
                            if (index9 == 11)
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 56, ty + 40, true);
                            if (index9 == 12)
                            {
                              this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 45, ty + 30, true);
                              continue;
                            }
                            continue;
                          }
                          this.DrawUnit(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 0 + 14 - num16 * 2 + index9 * 4 + 26, ty + 0 + 5 - num16 * 2 + index9 * 4 + 24, true, mostlyHidden: (index1 > 0));
                          continue;
                        }
                        this.DrawUnitBig(this.game.Data.MapObj[cmap].HexObj[cx, cy].UnitList[index1], forcehighlight, toG, tx + 0 + 28 - num16 * 1 + index9 * 2 + 0, ty + 0 + 10 - num16 * 2 + index9 * 4 + 0, true, mostlyHidden: (index1 > 0));
                        continue;
                    }
                  }
                }
              }
            }
          }
          else if (!InfoMode)
          {
            if (cx == 104 & cy == 28)
              cx = cx;
            if (this.game.EditObj.HisForce[cmap].Value[cx, cy] > -1 | this.game.EditObj.HisForce[cmap].Value[cx, cy] == -9999 | this.game.EditObj.HisForce[cmap].Value[cx, cy] > 0)
            {
              let mut num178: i32 =  this.game.EditObj.HisDepth[cmap].Value[cx, cy] - 1;
              let mut num179: i32 =  -1;
              num16 = this.game.EditObj.HisDepth[cmap].Value[cx, cy] - 1;
              let mut num180: i32 =  0;
              index9 = -1;
              if (num16 > 3)
              {
                num180 = num16 - 3;
                num16 = 3;
              }
              if (0 > num180)
                num180 = 0;
              for (index1 = num178; index1 >= 0; index1 += -1)
              {
                num179 += 1;
                if (num179 >= num180)
                {
                  index9 += 1;
                  if (this.game.EditObj.HisHis[cmap].Value[cx, cy] > -1 | this.game.EditObj.HisForce[cmap].Value[cx, cy] == -9999)
                  {
                    this.DrawUnit(-1, toG: toG, tx: (tx + 0 + 14 - num16 + index9 * 2), ty: (ty + 0 + 5 - num16 + index9 * 2), OverruleHis: this.game.EditObj.HisHis[cmap].Value[cx, cy], OverrulePower: this.game.EditObj.HisForce[cmap].Value[cx, cy], OverruleRegime: this.game.EditObj.HisOwner[cmap].Value[cx, cy]);
                  }
                  else
                  {
                    ref Graphics local319 = ref toG;
                    bitmap4 = this.DrawHistoryForce(this.game.EditObj.HisOwner[cmap].Value[cx, cy], this.game.EditObj.HisForce[cmap].Value[cx, cy], this.game.EditObj.HisSFType[cmap].Value[cx, cy]);
                    ref local320: Bitmap = ref bitmap4;
                    let mut x: i32 =  tx + 14;
                    let mut y: i32 =  ty + 5;
                    DrawMod.DrawSimple(ref local319, ref local320, x, y);
                  }
                }
              }
            }
          }
          if (!flag1 & this.game.EditObj.TownInfo &  this.game.Data.RuleVar[860] > 0.0 & this.game.Data.Turn > -1)
          {
            color1: Color = Color::new();
            color2: Color = Color::new();
            if (!InfoMode & this.game.Data.MapObj[cmap].HexObj[cx, cy].Location > -1)
            {
              if (this.game.Data.MapObj[0].HexObj[cx, cy].get_ReconPts(this.game.Data.Turn) > 0 | !this.game.Data.FOWOn)
              {
                let mut structuralPts: i32 =  this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Location].Type].StructuralPts;
                if (structuralPts > 0)
                {
                  float num181 =  this.game.Data.LocObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Location].StructuralPts /  structuralPts;
                  c1: Color = Color.Green;
                  c2: Color = Color.DarkGreen;
                  if ( num181 < 1.0)
                  {
                    c1 = Color.Green;
                    c2 = Color.Yellow;
                  }
                  if ( num181 < 0.75)
                  {
                    c1 = Color.Yellow;
                    c2 = Color.Blue;
                  }
                  if ( num181 < 0.5)
                  {
                    c1 = Color.Blue;
                    c2 = Color.Red;
                  }
                  if ( num181 < 0.25)
                  {
                    c1 = Color.Red;
                    c2 = Color.DarkRed;
                  }
                  switch (Zoom)
                  {
                    case -1:
                      DrawMod.DrawBlock(ref toG, tx + num6 - 6, ty + 8, 3, 8, 0, 0, 0, 100);
                      DrawMod.DrawBlockGradient2(ref toG, tx + num6 - 6, (int) Math.Round( ( (ty + 8) +  (8.0 -  num181 * 8.0))), 3, (int) Math.Round( (8f * num181)), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + num6 - 6, ty + 8, 3, 8, 0, 0, 0, (int) byte.MaxValue);
                      break;
                    case 0:
                      DrawMod.DrawBlock(ref toG, tx + num6 - 12, ty + 17, 5, 18, 0, 0, 0, 100);
                      DrawMod.DrawBlockGradient2(ref toG, tx + num6 - 12, (int) Math.Round( ( (ty + 17) +  (18.0 -  num181 * 18.0))), 5, (int) Math.Round( (18f * num181)), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + num6 - 12, ty + 17, 5, 18, 0, 0, 0, (int) byte.MaxValue);
                      break;
                    case 1:
                      DrawMod.DrawBlock(ref toG, tx + num6 - 21, ty + 36, 8, 36, 0, 0, 0, 100);
                      DrawMod.DrawBlockGradient2(ref toG, tx + num6 - 21, (int) Math.Round( ( (ty + 36) +  (36.0 -  num181 * 36.0))), 8, (int) Math.Round( (36f * num181)), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + num6 - 21, ty + 36, 8, 36, 0, 0, 0, (int) byte.MaxValue);
                      break;
                  }
                }
              }
              if (!this.game.Data.FOWOn | this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime, this.game.Data.Turn))
              {
                let mut hq2: i32 =  this.game.Data.LocObj[this.game.Data.MapObj[cmap].HexObj[cx, cy].Location].HQ;
                if (hq2 > -1)
                {
                  c1: Color = Color.FromArgb((int) byte.MaxValue, this.game.Data.UnitObj[hq2].Red, this.game.Data.UnitObj[hq2].Green, this.game.Data.UnitObj[hq2].Blue);
                  c2: Color = Color.FromArgb((int) byte.MaxValue, (int) Math.Round( this.game.Data.UnitObj[hq2].Red / 2.0), (int) Math.Round( this.game.Data.UnitObj[hq2].Green / 2.0), (int) Math.Round( this.game.Data.UnitObj[hq2].Blue / 2.0));
                  switch (Zoom)
                  {
                    case -1:
                      DrawMod.DrawBlockGradient2(ref toG, tx + 2, (int) Math.Round( ty +  num7 * 0.375), (int) Math.Round( num6 / 8.0), (int) Math.Round( num7 / 4.0), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + 2, (int) Math.Round( ty +  num7 * 0.375), (int) Math.Round( num6 / 8.0), (int) Math.Round( num7 / 4.0), 0, 0, 0, (int) byte.MaxValue);
                      break;
                    case 0:
                      DrawMod.DrawBlockGradient2(ref toG, tx + 4, (int) Math.Round( (ty + 1) +  num7 * 0.375), (int) Math.Round( num6 / 8.0 - 2.0), (int) Math.Round( num7 / 4.0 - 2.0), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + 4, (int) Math.Round( (ty + 1) +  num7 * 0.375), (int) Math.Round( num6 / 8.0 - 2.0), (int) Math.Round( num7 / 4.0 - 2.0), 0, 0, 0, (int) byte.MaxValue);
                      break;
                    case 1:
                      DrawMod.DrawBlockGradient2(ref toG, tx + 8, (int) Math.Round( (ty + 1) +  num7 * 0.375), (int) Math.Round( num6 / 8.0 - 2.0), (int) Math.Round( num7 / 4.0 - 2.0), c1, c2);
                      DrawMod.DrawRectangle(ref toG, tx + 8, (int) Math.Round( (ty + 1) +  num7 * 0.375), (int) Math.Round( num6 / 8.0 - 2.0), (int) Math.Round( num7 / 4.0 - 2.0), 0, 0, 0, (int) byte.MaxValue);
                      break;
                  }
                }
              }
            }
          }
        }
        else
        {
          if (flag1)
          {
            ref Graphics local321 = ref toG;
            bitmap1 = BitmapStore.GetBitmap(this.game.NOHEX, Zoom);
            ref local322: Bitmap = ref bitmap1;
            let mut x10: i32 =  tx;
            let mut y10: i32 =  ty;
            DrawMod.DrawSimple(ref local321, ref local322, x10, y10);
            if (this.game.EditObj.HexRasterOn & !ispredrawing)
            {
              ref Graphics local323 = ref toG;
              bitmap1 = BitmapStore.GetBitmap(this.game.HEXRASTER, Zoom);
              ref local324: Bitmap = ref bitmap1;
              let mut x11: i32 =  tx;
              let mut y11: i32 =  ty;
              DrawMod.DrawSimple(ref local323, ref local324, x11, y11);
            }
            return this.tmpbmp3;
          }
          if (Information.IsNothing( tempg))
            return this.tmpbmp3;
          this.ThreadRelease();
          goto label_2498;
        }
      }
      if (num8 == 0 & this.game.Data.ShrowdOn & this.game.EditObj.RealRound == 0 & !flag1 & !InfoMode)
      {
        ref Graphics local325 = ref toG;
        bitmap4 = BitmapStore.GetBitmap(this.game.NOHEX, Zoom);
        ref local326: Bitmap = ref bitmap4;
        let mut x: i32 =  tx;
        let mut y: i32 =  ty;
        DrawMod.DrawSimple(ref local325, ref local326, x, y);
      }
      else if (num8 == 0 & this.game.Data.ShrowdOn & this.game.EditObj.RealRound > 0 & !flag1 & !InfoMode)
      {
        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_SeeNow(index5) < 1)
        {
          let mut num182: i32 =  0;
          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5) == -1)
          {
            ref Graphics local327 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.NOHEX, Zoom);
            ref local328: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local327, ref local328, x, y);
          }
          else
          {
            num182 = 1;
            index6 = this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5);
            let mut index123: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastSpr(index5);
            if (!this.game.Data.LandscapeTypeObj[index6].Transparent)
            {
              if (index6 > -1)
                nr1 = this.game.Data.LandscapeTypeObj[index6].PreHexPicID;
              let mut num183: i32 =  (int) Math.Round( BitmapStore.GetBitmap(nr1, Zoom).Width /  num6);
              let mut num184: i32 =  0;
              if (num183 > 1)
                num184 = new Random((int) Math.Round(a1)).Next(0, num183 - 1);
              ref Graphics local329 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(nr1, Zoom);
              ref local330: Bitmap = ref bitmap4;
              rectangle2 = Rectangle::new(num184 * num6, 0, num6, num7);
              let mut srcrect15: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, num6, num7);
              let mut destrect15: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local329, ref local330, srcrect15, destrect15);
              let mut num185: i32 =  (int) Math.Round( BitmapStore.GetWidth(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID[index123], Zoom) /  num6);
              let mut num186: i32 =  0;
              if (num185 > 1)
                num186 = new Random((int) Math.Round(a1)).Next(0, num185 - 1);
              ref Graphics local331 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID[index123], Zoom);
              ref local332: Bitmap = ref bitmap4;
              rectangle2 = Rectangle::new(num186 * num6, 0, num6, num7);
              let mut srcrect16: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx, ty, num6, num7);
              let mut destrect16: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2(ref local331, ref local332, srcrect16, destrect16);
              if (this.game.Data.LandscapeTypeObj[index6].BasicSpriteID2[index123] > 0 & this.game.Data.LandscapeTypeObj[index6].PlotLast[index123])
              {
                ref Graphics local333 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.Data.LandscapeTypeObj[index6].BasicSpriteID2[index123], Zoom);
                ref local334: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num186 * num6, 0, num6, num7);
                let mut srcrect17: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect17: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local333, ref local334, srcrect17, destrect17);
              }
            }
          }
          if (num182 == 1)
          {
            index1 = 0;
            do
            {
              if (this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1] > -1)
              {
                let mut index124: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index1];
                if (!this.game.Data.RiverTypeObj[index124].Transparent)
                {
                  let mut num187: i32 =  (int) Math.Round( BitmapStore.GetWidth(this.game.Data.RiverTypeObj[index124].BasicSpriteID[index1], Zoom) /  num6);
                  num16 = 0;
                  if (!this.game.Data.RiverTypeObj[index124].snakeMode)
                  {
                    coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index1);
                    if (num187 > 1)
                      num16 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num187 - 1) : new Random((int) Math.Round(a1)).Next(0, num187 - 1);
                  }
                  else
                  {
                    index9 = index1 - 1;
                    index10 = index1 + 1;
                    if (index9 < 0)
                      index9 = 5;
                    if (index10 > 5)
                      index10 = 0;
                    index8 = this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index9];
                    let mut num188: i32 =  this.game.Data.MapObj[cmap].HexObj[cx, cy].RiverType[index10];
                    if (index8 == index124 & num188 == index124)
                      num16 = 0;
                    if (index8 == index124 & num188 != index124)
                      num16 = 1;
                    if (index8 != index124 & num188 == index124)
                      num16 = 2;
                    if (index8 != index124 & num188 != index124)
                      num16 = 3;
                  }
                  ref Graphics local335 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.Data.RiverTypeObj[index124].BasicSpriteID[index1], Zoom);
                  ref local336: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num16 * num6, 0, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2(ref local335, ref local336, srcrect, destrect);
                }
              }
              index1 += 1;
            }
            while (index1 <= 5);
          }
          if (flag2 & !this.game.Data.LandscapeTypeObj[index6].IsSea | num182 == 0 & flag2)
          {
            ref Graphics local337 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.WHITEHEXTRANS, Zoom);
            ref local338: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local337, ref local338, x, y);
            num15 = 1;
          }
          else if (num182 == 1 & flag2 & this.game.Data.LandscapeTypeObj[index6].IsSea)
          {
            ref Graphics local339 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.ROUNDBALL, Zoom);
            ref local340: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local339, ref local340, x, y);
          }
          if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(index5) != -1 & this.game.EditObj.ShowLabel)
          {
            let mut num189: i32 =  0;
            if (this.game.Data.Turn > -1)
            {
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].Regime == index5)
                num189 = 1;
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].get_ReconPts(this.game.EditObj.RealTurn) >= 1)
                num189 = 1;
              if (this.game.Data.MapObj[coordinate1.map].HexObj[cx, cy].MaxRecon >= 1 & !flag1)
                num189 = 1;
            }
            else
              num189 = 1;
            if (!this.game.Data.ShrowdOn)
              num189 = 1;
            if (num189 == 1 && !ispredrawing && !ispredrawing)
            {
              if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 <= 5)
                {
                  c: Color = Color.Transparent;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 1)
                    c = Color.White;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 2)
                    c = Color.Blue;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 3)
                    c = Color.Black;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 4)
                    c = Color.Red;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 5)
                    c = Color.Yellow;
                  if (Zoom == 0)
                  {
                    str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 6, c, true);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 6, c, true);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 27, ty + 6, c, true);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 37, ty + 6, c, true);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 48, ty + 6, c, true);
                  }
                  else if ( this.game.Data.RuleVar[352] == 0.0 & Zoom == 1)
                  {
                    index1 = 1;
                    do
                    {
                      str: String = Strings.UCase(Strings.Mid(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, index1, 1));
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredOutline(ref toG, str, Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 16 + 20 * (index1 - 1), ty + 12, c, true);
                      index1 += 1;
                    }
                    while (index1 <= 5);
                  }
                  else if (Zoom == -1)
                  {
                    str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 2, ty + 0, c);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 0, c);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 12, ty + 0, c);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 0, c);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 9f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 22, ty + 0, c);
                  }
                }
                else
                {
                  switch (Zoom)
                  {
                    case -1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
                      {
                        str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 1));
                        SizeF sizeF8 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 16) - sizeF8.Width / 2f)), ty - 3, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 0:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1) > 0)
                      {
                        str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText1, 1));
                        SizeF sizeF9 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType1 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 32) - sizeF9.Width / 2f)), ty + 4, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                  }
                }
              }
              if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
              {
                if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 <= 5)
                {
                  c: Color = Color.Transparent;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 1)
                    c = Color.White;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 2)
                    c = Color.Blue;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 3)
                    c = Color.Black;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 4)
                    c = Color.Red;
                  if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 5)
                    c = Color.Yellow;
                  if (Zoom == 0)
                  {
                    str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 7, ty + 30, c, true);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 17, ty + 30, c, true);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 27, ty + 30, c, true);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 37, ty + 30, c, true);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 12f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 48, ty + 30, c, true);
                  }
                  else if ( this.game.Data.RuleVar[352] == 0.0 & Zoom == 1)
                  {
                    index1 = 1;
                    do
                    {
                      str: String = Strings.UCase(Strings.Mid(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, index1, 1));
                      if (Strings.Len(str) >= 1)
                        DrawMod.DrawTextColouredOutline(ref toG, str, Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 16 + 20 * (index1 - 1), ty + 12 + 48, c, true);
                      index1 += 1;
                    }
                    while (index1 <= 5);
                  }
                  else if (Zoom == -1)
                  {
                    str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 5));
                    if (Strings.Len(str) >= 1)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 1, ty + 12, c);
                    if (Strings.Len(str) >= 2)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 2, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 6, ty + 12, c);
                    if (Strings.Len(str) >= 3)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 3, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 11, ty + 12, c);
                    if (Strings.Len(str) >= 4)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 4, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 16, ty + 12, c);
                    if (Strings.Len(str) >= 5)
                      DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 5, 1), Font::new("Courier New", 9f, FontStyle.Regular, GraphicsUnit.Pixel), tx + 21, ty + 12, c);
                  }
                }
                else
                {
                  switch (Zoom)
                  {
                    case -1:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
                      {
                        str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 1));
                        SizeF sizeF10 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 12f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 16) - sizeF10.Width / 2f)), ty + 9, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                    case 0:
                      if (Strings.Len(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2) > 0)
                      {
                        str: String = Strings.UCase(Strings.Left(this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelText2, 1));
                        SizeF sizeF11 = toG.MeasureString(Strings.Left(str, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel));
                        c: Color = Color.Transparent;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 6)
                          c = Color.White;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 7)
                          c = Color.Blue;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 8)
                          c = Color.Black;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 9)
                          c = Color.Red;
                        if (this.game.Data.MapObj[cmap].HexObj[cx, cy].LabelType2 == 10)
                          c = Color.Yellow;
                        if (Strings.Len(str) >= 1)
                        {
                          DrawMod.DrawTextColouredOutline(ref toG, Strings.Mid(str, 1, 1), Font::new("Arial Black", 16f, FontStyle.Bold, GraphicsUnit.Pixel), (int) Math.Round( ( (tx + 32) - sizeF11.Width / 2f)), ty + 28, c, true);
                          break;
                        }
                        break;
                      }
                      break;
                  }
                }
              }
            }
          }
          if (num182 == 1 & !NoShader)
          {
            ref Graphics local341 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.SHADEDHEX, Zoom);
            ref local342: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.DrawSimple(ref local341, ref local342, x, y);
          }
        }
        else
        {
          ref Graphics local343 = ref toG;
          bitmap4 = BitmapStore.GetBitmap(this.game.NOHEX, Zoom);
          ref local344: Bitmap = ref bitmap4;
          let mut x: i32 =  tx;
          let mut y: i32 =  ty;
          DrawMod.DrawSimple(ref local343, ref local344, x, y);
        }
      }
      if (!this.game.EditObj.AIMoving & !InfoMode && this.game.EditObj.OrderType == 25 & this.game.EditObj.OrderSubType == 1)
      {
        if (this.game.EditObj.TempCoordList.counter > 0)
        {
          let mut counter: i32 =  this.game.EditObj.TempCoordList.counter;
          for (index1 = 0; index1 <= counter; index1 += 1)
          {
            coordinate2 = this.game.EditObj.TempCoordList.coord[index1];
            Coordinate coordinate3;
            if (coordinate2.x == cx & coordinate2.y == cy & index1 < this.game.EditObj.TempCoordList.counter)
            {
              coordinate3 = this.game.EditObj.TempCoordList.coord[index1 + 1];
              let mut index125: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate2.x, coordinate2.y, coordinate2.map, coordinate3.x, coordinate3.y, coordinate3.map) - 1;
              let mut index126: i32 =  (int) Math.Round( this.game.Data.RuleVar[32]);
              ref Graphics local345 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.Data.RoadTypeObj[index126].BasicSpriteID[index125], Zoom);
              ref local346: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.Draw(ref local345, ref local346, x, y, 0.6f, 0.2f, 0.6f, 0.6f);
            }
            if (coordinate2.x == cx & coordinate2.y == cy & index1 > 0)
            {
              coordinate3 = this.game.EditObj.TempCoordList.coord[index1 - 1];
              let mut index127: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate2.x, coordinate2.y, coordinate2.map, coordinate3.x, coordinate3.y, coordinate3.map) - 1;
              let mut index128: i32 =  (int) Math.Round( this.game.Data.RuleVar[32]);
              ref Graphics local347 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.Data.RoadTypeObj[index128].BasicSpriteID[index127], Zoom);
              ref local348: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.Draw(ref local347, ref local348, x, y, 0.6f, 0.2f, 0.6f, 0.6f);
            }
          }
        }
        else if (this.game.EditObj.TempCoordList.counter == 0)
        {
          coordinate2 = this.game.EditObj.TempCoordList.coord[this.game.EditObj.TempCoordList.counter];
          if (coordinate2.x == cx & coordinate2.y == cy)
          {
            let mut index129: i32 =  (int) Math.Round( this.game.Data.RuleVar[32]);
            ref Graphics local349 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.Data.RoadTypeObj[index129].BasicSpriteID[0], Zoom);
            ref local350: Bitmap = ref bitmap4;
            let mut x: i32 =  tx;
            let mut y: i32 =  ty;
            DrawMod.Draw(ref local349, ref local350, x, y, 0.6f, 0.2f, 0.6f, 0.6f);
          }
        }
      }
      if (!this.game.EditObj.AIMoving & !InfoMode && flag3)
      {
        if (Information.IsNothing( this.game.EditObj.TempValueSpecial))
          this.game.HandyFunctionsObj.RedimTempValueSpecial(0);
        if (this.game.EditObj.TempValueSpecial[cmap].Value[cx, cy] == 1 | this.game.EditObj.TempValueSpecial[cmap].Value[cx, cy] == 3)
        {
          switch (Zoom)
          {
            case -1:
              ref Graphics local351 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
              ref local352: Bitmap = ref bitmap4;
              let mut x12: i32 =  tx - 10;
              let mut y12: i32 =  ty;
              DrawMod.DrawSimple(ref local351, ref local352, x12, y12);
              break;
            case 0:
              ref Graphics local353 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
              ref local354: Bitmap = ref bitmap4;
              let mut x13: i32 =  (int) Math.Round( (tx - 5) +  num6 / 4.0);
              let mut y13: i32 =  (int) Math.Round( ty +  num7 / 4.0);
              DrawMod.DrawSimple(ref local353, ref local354, x13, y13);
              break;
            default:
              ref Graphics local355 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
              ref local356: Bitmap = ref bitmap4;
              let mut x14: i32 =  (int) Math.Round( tx +  num6 / 4.0 + 10.0);
              let mut y14: i32 =  (int) Math.Round( ty +  num7 / 4.0 + 8.0);
              DrawMod.DrawSimple(ref local355, ref local356, x14, y14);
              break;
          }
        }
      }
      if (flag1 & !InfoMode)
      {
        index1 = 0;
        do
        {
          if (this.game.EditObj.HisHotX == cx & this.game.EditObj.HisHotY == cy & this.game.EditObj.HisHotMap == cmap)
          {
            if (this.game.EditObj.HisAttackType == 2 | this.game.EditObj.HisAttackType == 12)
            {
              if (this.game.EditObj.HisNeighbour.data[0] > 0)
              {
                ref Graphics local357 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK0, Zoom);
                ref local358: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local357, ref local358, x, y);
              }
              if (this.game.EditObj.HisNeighbour.data[1] > 0)
              {
                ref Graphics local359 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK1, Zoom);
                ref local360: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local359, ref local360, x, y);
              }
              if (this.game.EditObj.HisNeighbour.data[2] > 0)
              {
                ref Graphics local361 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK2, Zoom);
                ref local362: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local361, ref local362, x, y);
              }
              if (this.game.EditObj.HisNeighbour.data[3] > 0)
              {
                ref Graphics local363 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK3, Zoom);
                ref local364: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local363, ref local364, x, y);
              }
              if (this.game.EditObj.HisNeighbour.data[4] > 0)
              {
                ref Graphics local365 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK4, Zoom);
                ref local366: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local365, ref local366, x, y);
              }
              if (this.game.EditObj.HisNeighbour.data[5] > 0)
              {
                ref Graphics local367 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK5, Zoom);
                ref local368: Bitmap = ref bitmap4;
                let mut x: i32 =  tx;
                let mut y: i32 =  ty;
                DrawMod.DrawSimple(ref local367, ref local368, x, y);
              }
            }
            if (this.game.EditObj.HisAttackType == 21)
            {
              ref Graphics local369 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACKAMPH, Zoom);
              ref local370: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local369, ref local370, x, y);
            }
            if (this.game.EditObj.HisAttackType == 19)
            {
              ref Graphics local371 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACKPARADROP, Zoom);
              ref local372: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local371, ref local372, x, y);
            }
            if (this.game.EditObj.HisAttackType == 11 | this.game.EditObj.HisAttackType == 13)
            {
              ref Graphics local373 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACKART, Zoom);
              ref local374: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local373, ref local374, x, y);
            }
            if (this.game.EditObj.HisAttackType == 55 | this.game.EditObj.HisAttackType == 14 | this.game.EditObj.HisAttackType == 15 | this.game.EditObj.HisAttackType == 17)
            {
              ref Graphics local375 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACKAIR, Zoom);
              ref local376: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local375, ref local376, x, y);
            }
            if (this.game.EditObj.HisAttackType == 31)
            {
              ref Graphics local377 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.TARGETHEX, Zoom);
              ref local378: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local377, ref local378, x, y);
            }
            if (this.game.EditObj.HisAttackType == 18)
            {
              ref Graphics local379 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACKAIR, Zoom);
              ref local380: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local379, ref local380, x, y);
            }
            if (this.game.EditObj.HisAttackType < 1)
            {
              ref Graphics local381 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.SELECTEDHEX, Zoom);
              ref local382: Bitmap = ref bitmap4;
              let mut x: i32 =  tx;
              let mut y: i32 =  ty;
              DrawMod.DrawSimple(ref local381, ref local382, x, y);
            }
          }
          index1 += 1;
        }
        while (index1 <= 5);
      }
      if (!this.game.EditObj.AIMoving & !InfoMode && this.game.EditObj.OrderType == 40 &&  this.game.Data.RuleVar[858] >=  this.game.EditObj.TempValue3[0].Value[cx, cy])
      {
        switch (Zoom)
        {
          case -1:
            ref Graphics local383 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
            ref local384: Bitmap = ref bitmap4;
            let mut x15: i32 =  tx - 10;
            let mut y15: i32 =  ty;
            DrawMod.DrawSimple(ref local383, ref local384, x15, y15);
            break;
          case 0:
            ref Graphics local385 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
            ref local386: Bitmap = ref bitmap4;
            let mut x16: i32 =  (int) Math.Round( (tx - 5) +  num6 / 4.0);
            let mut y16: i32 =  (int) Math.Round( ty +  num7 / 4.0);
            DrawMod.DrawSimple(ref local385, ref local386, x16, y16);
            break;
          default:
            ref Graphics local387 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.SUPPLIESSYMBOL);
            ref local388: Bitmap = ref bitmap4;
            let mut x17: i32 =  (int) Math.Round( tx +  num6 / 4.0 + 10.0);
            let mut y17: i32 =  (int) Math.Round( ty +  num7 / 4.0 + 8.0);
            DrawMod.DrawSimple(ref local387, ref local388, x17, y17);
            break;
        }
      }
      if (!this.game.EditObj.AIMoving & !InfoMode && (this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48) & this.game.Data.Round > 0)
      {
        index1 = 0;
        do
        {
          if (this.game.EditObj.TempAttack[cmap].Value[cx, cy, index1])
          {
            if (index1 == 0)
            {
              ref Graphics local389 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK0, Zoom);
              ref local390: Bitmap = ref bitmap4;
              let mut x18: i32 =  tx;
              let mut y18: i32 =  ty;
              DrawMod.DrawSimple(ref local389, ref local390, x18, y18);
            }
            if (index1 == 1)
            {
              ref Graphics local391 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK1, Zoom);
              ref local392: Bitmap = ref bitmap4;
              let mut x19: i32 =  tx;
              let mut y19: i32 =  ty;
              DrawMod.DrawSimple(ref local391, ref local392, x19, y19);
            }
            if (index1 == 2)
            {
              ref Graphics local393 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK2, Zoom);
              ref local394: Bitmap = ref bitmap4;
              let mut x20: i32 =  tx;
              let mut y20: i32 =  ty;
              DrawMod.DrawSimple(ref local393, ref local394, x20, y20);
            }
            if (index1 == 3)
            {
              ref Graphics local395 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK3, Zoom);
              ref local396: Bitmap = ref bitmap4;
              let mut x21: i32 =  tx;
              let mut y21: i32 =  ty;
              DrawMod.DrawSimple(ref local395, ref local396, x21, y21);
            }
            if (index1 == 4)
            {
              ref Graphics local397 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK4, Zoom);
              ref local398: Bitmap = ref bitmap4;
              let mut x22: i32 =  tx;
              let mut y22: i32 =  ty;
              DrawMod.DrawSimple(ref local397, ref local398, x22, y22);
            }
            if (index1 == 5)
            {
              ref Graphics local399 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(this.game.ATTACK5, Zoom);
              ref local400: Bitmap = ref bitmap4;
              let mut x23: i32 =  tx;
              let mut y23: i32 =  ty;
              DrawMod.DrawSimple(ref local399, ref local400, x23, y23);
            }
          }
          index1 += 1;
        }
        while (index1 <= 5);
      }
      if (this.game.EditObj.HexRasterOn & !ispredrawing)
      {
        ref Graphics local401 = ref toG;
        bitmap4 = BitmapStore.GetBitmap(this.game.HEXRASTER, Zoom);
        ref local402: Bitmap = ref bitmap4;
        let mut x24: i32 =  tx;
        let mut y24: i32 =  ty;
        DrawMod.DrawSimple(ref local401, ref local402, x24, y24);
      }
      if (index6 > -1 &&  this.game.Data.RuleVar[330] == 1.0 & !this.game.Data.LandscapeTypeObj[index6].BlackedOut & !InfoMode)
      {
        let mut num190: i32 =  1;
        do
        {
          if (num190 == 1)
            index1 = 2;
          if (num190 == 2)
            index1 = 3;
          if (num190 == 3)
            index1 = 6;
          if (num190 == 4)
            index1 = 5;
          if (num190 == 5)
            index1 = 4;
          if (num190 == 6)
            index1 = 1;
          coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, cmap, index1);
          if (!coordinate1.onmap && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate1.map].HexObj[coordinate1.x, coordinate1.y].LandscapeType].BlackedOut)
          {
            if (cx % 2 > 0 & index1 == 1)
              index1 = 7;
            if (cx != 0 & cx % 2 == 0 & index1 == 4)
              index1 = 8;
            if (cy == 0 & cx != 0 & cx % 2 == 0 & index1 == 6)
              index1 = 11;
            if (cx != this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth & cy == this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight & index1 == 3)
              index1 = 9;
            if (cx != 0 & cy == this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight & index1 == 5)
              index1 = 10;
            ref Graphics local403 = ref toG;
            bitmap4 = BitmapStore.GetBitmap(this.game.MAPBORDER[index1 - 1], Zoom);
            ref local404: Bitmap = ref bitmap4;
            let mut x25: i32 =  tx;
            let mut y25: i32 =  ty;
            DrawMod.DrawSimple(ref local403, ref local404, x25, y25);
          }
          num190 += 1;
        }
        while (num190 <= 6);
      }
      if (!InfoMode & !this.game.EditObj.AIMoving & !flag1)
      {
        if (this.game.SelectX == cx & this.game.SelectY == cy)
        {
          ref Graphics local405 = ref toG;
          bitmap4 = BitmapStore.GetBitmap(this.game.SELECTEDHEX, Zoom);
          ref local406: Bitmap = ref bitmap4;
          let mut x26: i32 =  tx;
          let mut y26: i32 =  ty;
          DrawMod.DrawSimple(ref local405, ref local406, x26, y26);
        }
        if (this.game.EditObj.TargetX == cx & this.game.EditObj.TargetY == cy & (this.game.EditObj.OrderType == 2 | this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 13 | this.game.EditObj.OrderType == 12))
        {
          ref Graphics local407 = ref toG;
          bitmap4 = BitmapStore.GetBitmap(this.game.TARGETHEX, Zoom);
          ref local408: Bitmap = ref bitmap4;
          let mut x27: i32 =  tx;
          let mut y27: i32 =  ty;
          DrawMod.DrawSimple(ref local407, ref local408, x27, y27);
        }
        if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(this.game.EditObj.TutX,  cx, false), Operators.CompareObjectEqual(this.game.EditObj.TutY,  cy, false))))
        {
          ref Graphics local409 = ref toG;
          bitmap4 = BitmapStore.GetBitmap(this.game.TUTHEX, Zoom);
          ref local410: Bitmap = ref bitmap4;
          let mut x28: i32 =  tx;
          let mut y28: i32 =  ty;
          DrawMod.DrawSimple(ref local409, ref local410, x28, y28);
        }
      }
      if (!this.game.EditObj.AIMoving & !InfoMode &&  this.game.Data.RuleVar[540] == 1.0 & this.game.EditObj.mouseOverActive)
      {
        int[] numArray33 = new int[7];
        if (this.game.EditObj.OrderType == 48 & this.game.Data.Product >= 6 & flag2 && this.game.EditObj.tempGroupMoveCounter > -1)
        {
          let mut groupMoveCounter: i32 =  this.game.EditObj.tempGroupMoveCounter;
          for (let mut index130: i32 =  0; index130 <= groupMoveCounter; index130 += 1)
          {
            if (!Information.IsNothing( this.game.EditObj.tempGroupMovePath[index130]))
            {
              let mut slot: i32 =  this.game.EditObj.tempGroupMovePath[index130].FindSlot(cx, cy, 0);
              if (slot > -1 & slot < this.game.EditObj.tempGroupMovePath[index130].counter)
              {
                let mut num191: i32 =  -1;
                let mut num192: i32 =  -1;
                if (slot > 0 && this.game.EditObj.TempValue[0].Value[this.game.EditObj.tempGroupMovePath[index130].coord[slot - 1].x, this.game.EditObj.tempGroupMovePath[index130].coord[slot - 1].y] < 9999)
                  num192 = this.game.HandyFunctionsObj.HexFacing(cx, cy, 0, this.game.EditObj.tempGroupMovePath[index130].coord[slot - 1].x, this.game.EditObj.tempGroupMovePath[index130].coord[slot - 1].y, 0);
                if (slot < this.game.EditObj.tempGroupMovePath[index130].counter)
                  num191 = this.game.HandyFunctionsObj.HexFacing(cx, cy, 0, this.game.EditObj.tempGroupMovePath[index130].coord[slot + 1].x, this.game.EditObj.tempGroupMovePath[index130].coord[slot + 1].y, 0);
                numArray33[0] = 0;
                numArray33[1] = 0;
                numArray33[2] = 0;
                numArray33[3] = 0;
                numArray33[4] = 0;
                numArray33[5] = 0;
                if (num191 > -1)
                  numArray33[num191 - 1] = 1;
                if (num192 > -1)
                  numArray33[num192 - 1] = 2;
                let mut num193: i32 =  this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                if (num193 > 0)
                {
                  num16 = (int) Math.Round(Math.Floor( (num193 - 1) / 6.0));
                  let mut num194: i32 =  num193 - num16 * 6 - 1;
                  ref Graphics local411 = ref toG;
                  bitmap4 = BitmapStore.GetBitmap(this.game.ARROWSHEET, Zoom);
                  ref local412: Bitmap = ref bitmap4;
                  rectangle2 = Rectangle::new(num194 * num6, num16 * num7, num6, num7);
                  let mut srcrect: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(tx, ty, num6, num7);
                  let mut destrect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2ColouredNew(ref local411, ref local412, srcrect, destrect, 1f, 1f, 1f, 0.6f);
                }
              }
            }
          }
        }
        if (!Information.IsNothing( this.game.EditObj.TempMovePathList) & this.game.EditObj.OrderType != 48)
        {
          let mut slot: i32 =  this.game.EditObj.TempMovePathList.FindSlot(cx, cy, 0);
          if (slot > -1 & slot <= this.game.EditObj.TempMovePathList.counter & this.game.EditObj.MouseOverX > -1 & this.game.EditObj.MouseOverY > -1 && this.game.EditObj.TempValue[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] < 9999)
          {
            let mut num195: i32 =  -1;
            let mut num196: i32 =  -1;
            if (slot > 0)
              num196 = this.game.HandyFunctionsObj.HexFacing(cx, cy, 0, this.game.EditObj.TempMovePathList.coord[slot - 1].x, this.game.EditObj.TempMovePathList.coord[slot - 1].y, 0);
            if (slot < this.game.EditObj.TempMovePathList.counter)
              num195 = this.game.HandyFunctionsObj.HexFacing(cx, cy, 0, this.game.EditObj.TempMovePathList.coord[slot + 1].x, this.game.EditObj.TempMovePathList.coord[slot + 1].y, 0);
            if (num195 > -1)
              numArray33[num195 - 1] = 1;
            if (num196 > -1)
              numArray33[num196 - 1] = 2;
            let mut num197: i32 =  this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
            if (num196 > -1)
              numArray33[num196 - 1] = 2;
            if (cx == 31 & cy == 17)
              cx = cx;
            bool flag27 = false;
            bool flag28 = false;
            if (this.game.EditObj.OrderType == 18 && (int) Math.Round(Conversion.Val(this.game.Data.Designer)) >= 82)
            {
              if (!flag27 & slot > 0 & num196 > 0)
              {
                let mut x29: i32 =  this.game.EditObj.TempMovePathList.coord[slot - 1].x;
                let mut y29: i32 =  this.game.EditObj.TempMovePathList.coord[slot - 1].y;
                if (this.game.EditObj.TempCameFrom[0].Value[x29, y29].onmap & this.game.EditObj.TempCameFrom[0].Value[x29, y29].data1 > 0)
                {
                  if (this.game.EditObj.TempCameFrom[0].Value[x29, y29].data1 != this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1)
                  {
                    numArray33[num196 - 1] = 0;
                    num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                    flag28 = true;
                  }
                  else if (this.game.EditObj.TempCameFrom[0].Value[x29, y29].data1 > 0 & !this.game.EditObj.TempCameFrom[0].Value[cx, cy].onmap)
                  {
                    numArray33[num196 - 1] = 0;
                    num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                    flag28 = true;
                  }
                }
                else if (this.game.EditObj.TempCameFrom[0].Value[x29, y29].data1 > 0 & !this.game.EditObj.TempCameFrom[0].Value[x29, y29].onmap)
                {
                  numArray33[num196 - 1] = 0;
                  num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                  flag28 = true;
                }
              }
              if (!flag27 & slot < this.game.EditObj.TempMovePathList.counter & num195 > 0)
              {
                let mut x30: i32 =  this.game.EditObj.TempMovePathList.coord[slot + 1].x;
                let mut y30: i32 =  this.game.EditObj.TempMovePathList.coord[slot + 1].y;
                if (this.game.EditObj.TempCameFrom[0].Value[cx, cy].onmap & this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1 > 0)
                {
                  if (this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1 != this.game.EditObj.TempCameFrom[0].Value[x30, y30].data1)
                  {
                    numArray33[num195 - 1] = 0;
                    num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                    flag28 = true;
                  }
                  else if (this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1 > 0 & !this.game.EditObj.TempCameFrom[0].Value[x30, y30].onmap)
                  {
                    numArray33[num195 - 1] = 0;
                    num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                    flag28 = true;
                  }
                }
                else if (this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1 > 0 & !this.game.EditObj.TempCameFrom[0].Value[x30, y30].onmap)
                {
                  numArray33[num195 - 1] = 0;
                  num197 = this.game.ARROW64[numArray33[0], numArray33[1], numArray33[2], numArray33[3], numArray33[4], numArray33[5]];
                  flag28 = true;
                }
              }
              if (num197 == 0 & num195 == -1 & num196 == -1 && this.game.EditObj.TempCameFrom[0].Value[cx, cy].data2 > 0 & this.game.EditObj.TempCameFrom[0].Value[cx, cy].data1 > 0)
                flag28 = true;
              if (num197 > 0 & (num195 == -1 | num196 == -1) & (num195 > -1 | num196 > -1) && slot > 0 && this.game.EditObj.TempCameFrom[0].Value[this.game.EditObj.TempMovePathList.coord[slot - 1].x, this.game.EditObj.TempMovePathList.coord[slot - 1].y].data2 > 0)
                flag28 = true;
            }
            if (flag28)
            {
              ref Graphics local413 = ref toG;
              bitmap4 = BitmapStore.GetBitmap(DrawMod.TGame.SE1_ICONS);
              ref local414: Bitmap = ref bitmap4;
              rectangle2 = Rectangle::new(2604, 0, 42, 32);
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(tx + (int) Math.Round( num6 * 0.125), ty + (int) Math.Round( num7 * 0.166), (int) Math.Round( num6 * 0.75), (int) Math.Round( num7 * 0.66));
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2ColouredOld(ref local413, ref local414, srcrect, destrect, 1f, 1f, 1f, 1f);
            }
            if (num197 > 0)
            {
              num16 = (int) Math.Round(Math.Floor( (num197 - 1) / 6.0));
              let mut num198: i32 =  num197 - num16 * 6 - 1;
              if (!flag27 & num197 > 0)
              {
                ref Graphics local415 = ref toG;
                bitmap4 = BitmapStore.GetBitmap(this.game.ARROWSHEET, Zoom);
                ref local416: Bitmap = ref bitmap4;
                rectangle2 = Rectangle::new(num198 * num6, num16 * num7, num6, num7);
                let mut srcrect: &Rectangle = &rectangle2
                rectangle1 = Rectangle::new(tx, ty, num6, num7);
                let mut destrect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart2(ref local415, ref local416, srcrect, destrect);
              }
              if (this.game.EditObj.OrderType == 36)
              {
                if (this.game.EditObj.OrderSubType == 8)
                {
                  let mut num199: i32 =  0;
                  let mut index131: i32 =  0;
                  do
                  {
                    if (numArray33[index131] > 0)
                    {
                      let mut nr21: i32 =  this.game.Data.RoadTypeObj[0].BasicSpriteID[index131];
                      index10 = (int) Math.Round( BitmapStore.GetWidth(nr21, Zoom) /  num6);
                      index8 = 0;
                      num199 += 1;
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index131 + 1);
                      if (index10 > 1)
                        index8 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num198 - 1) : new Random((int) Math.Round(a1)).Next(0, num198 - 1);
                      ref Graphics local417 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(nr21, Zoom);
                      ref local418: Bitmap = ref bitmap4;
                      rectangle2 = Rectangle::new(index8 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2ColouredNew(ref local417, ref local418, srcrect, destrect, 1f, 0.0f, 0.0f, 1f);
                    }
                    index131 += 1;
                  }
                  while (index131 <= 5);
                  if (num199 > 1)
                  {
                    let mut center6spriteId: i32 =  this.game.Data.RoadTypeObj[0].center6spriteId;
                    if (center6spriteId > -1)
                    {
                      ref Graphics local419 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(center6spriteId, Zoom);
                      ref local420: Bitmap = ref bitmap4;
                      rectangle2 = Rectangle::new(index8 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2ColouredNew(ref local419, ref local420, srcrect, destrect, 1f, 0.0f, 0.0f, 1f);
                    }
                  }
                }
                else if (this.game.Data.RoadTypeObj[this.game.EditObj.OrderSubType].UseSheet)
                {
                  int[] numArray34 = new int[7];
                  index9 = 0;
                  let mut index132: i32 =  0;
                  do
                  {
                    if (numArray33[index132] > 0)
                    {
                      if (this.game.Data.MapObj[0].HexObj[cx, cy].RoadType[index132] < this.game.EditObj.OrderSubType)
                      {
                        numArray34[index132] = 1;
                        index9 = 1;
                      }
                      else if (this.game.Data.MapObj[0].HexObj[cx, cy].RoadType[index132] == 2 & this.game.EditObj.OrderSubType == 0)
                      {
                        numArray34[index132] = 1;
                        index9 = 1;
                      }
                      else if (this.game.Data.MapObj[0].HexObj[cx, cy].RoadType[index132] == 2 & this.game.EditObj.OrderSubType == 1)
                      {
                        numArray34[index132] = 1;
                        index9 = 1;
                      }
                      else if (this.game.Data.MapObj[0].HexObj[cx, cy].RoadType[index132] == 3 & this.game.EditObj.OrderSubType == 1)
                      {
                        numArray34[index132] = 1;
                        index9 = 1;
                      }
                    }
                    index132 += 1;
                  }
                  while (index132 <= 5);
                  if (index9 == 1)
                  {
                    let mut sheetSpriteId: i32 =  this.game.Data.RoadTypeObj[this.game.EditObj.OrderSubType].SheetSpriteID;
                    let mut index133: i32 =  this.game.SPRITE64[numArray34[0], numArray34[1], numArray34[2], numArray34[3], numArray34[4], numArray34[5]];
                    ref Graphics local421 = ref toG;
                    bitmap4 = BitmapStore.GetBitmap(sheetSpriteId, Zoom);
                    ref local422: Bitmap = ref bitmap4;
                    rectangle2 = Rectangle::new(this.game.SHEETX[index133] * num6, this.game.SHEETY[index133] * num7, num6, num7);
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(tx, ty, num6, num7);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2(ref local421, ref local422, srcrect, destrect);
                  }
                }
                else if (!this.game.Data.RoadTypeObj[this.game.EditObj.OrderSubType].SpecialLayer)
                {
                  let mut num200: i32 =  0;
                  let mut index134: i32 =  0;
                  do
                  {
                    if (numArray33[index134] > 0)
                    {
                      let mut nr22: i32 =  this.game.Data.RoadTypeObj[this.game.EditObj.OrderSubType].BasicSpriteID[index134];
                      index10 = (int) Math.Round( BitmapStore.GetWidth(nr22, Zoom) /  num6);
                      index8 = 0;
                      num200 += 1;
                      coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, index134 + 1);
                      if (index10 > 1)
                        index8 = !coordinate1.onmap ? new Random((int) Math.Round(a1)).Next(0, num198 - 1) : new Random((int) Math.Round(a1)).Next(0, num198 - 1);
                      ref Graphics local423 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(nr22, Zoom);
                      ref local424: Bitmap = ref bitmap4;
                      rectangle2 = Rectangle::new(index8 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local423, ref local424, srcrect, destrect);
                    }
                    index134 += 1;
                  }
                  while (index134 <= 5);
                  if (num200 > 1)
                  {
                    let mut center6spriteId: i32 =  this.game.Data.RoadTypeObj[this.game.EditObj.OrderSubType].center6spriteId;
                    if (center6spriteId > -1)
                    {
                      ref Graphics local425 = ref toG;
                      bitmap4 = BitmapStore.GetBitmap(center6spriteId, Zoom);
                      ref local426: Bitmap = ref bitmap4;
                      rectangle2 = Rectangle::new(index8 * num6, 0, num6, num7);
                      let mut srcrect: &Rectangle = &rectangle2
                      rectangle1 = Rectangle::new(tx, ty, num6, num7);
                      let mut destrect: &Rectangle = &rectangle1
                      DrawMod.DrawSimplePart2(ref local425, ref local426, srcrect, destrect);
                    }
                  }
                }
              }
            }
          }
        }
      }
      if (this.game.EditObj.RealRound == 0)
      {
        if (this.game.EditObj.PencilType == 9)
        {
          if (Zoom == -1)
          {
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].AreaCode[this.game.EditObj.PencilData1]), Font::new("Times New Roman", 10f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 8, ty + 3);
            str: String = this.game.HandyFunctionsObj.GetAreaName(this.game.EditObj.PencilData1, this.game.Data.MapObj[cmap].HexObj[cx, cy].AreaCode[this.game.EditObj.PencilData1]);
            if (Strings.Len(str) > 0)
            {
              if (Strings.Len(str) > 10)
                str = Strings.Left(str, 9) + ".";
              DrawMod.DrawText(ref toG, str, Font::new(this.game.FontCol.Families[1], 8f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 3, ty + 10);
            }
          }
          else
          {
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].AreaCode[this.game.EditObj.PencilData1]), Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 16, ty + 7);
            str: String = this.game.HandyFunctionsObj.GetAreaName(this.game.EditObj.PencilData1, this.game.Data.MapObj[cmap].HexObj[cx, cy].AreaCode[this.game.EditObj.PencilData1]);
            if (Strings.Len(str) > 0)
            {
              if (Strings.Len(str) > 10)
                str = Strings.Left(str, 9) + ".";
              DrawMod.DrawText(ref toG, str, Font::new(this.game.FontCol.Families[1], 10f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 6, ty + 22);
            }
          }
        }
        if (this.game.EditObj.PencilType == 11)
        {
          if (Zoom == -1)
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].GetHexLibVarValue(this.game.EditObj.PencilData1)), Font::new("Times New Roman", 10f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 8, ty + 3);
          else
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].GetHexLibVarValue(this.game.EditObj.PencilData1)), Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 16, ty + 7);
        }
        if (this.game.EditObj.PencilType == 12)
        {
          if (Zoom == -1)
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].HeightLevel), Font::new("Times New Roman", 10f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 8, ty + 3);
          else
            DrawMod.DrawText(ref toG, Conversion.Str( this.game.Data.MapObj[cmap].HexObj[cx, cy].HeightLevel), Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), tx + 16, ty + 7);
        }
      }
      this.ThreadRelease();
      if (Information.IsNothing( tempg))
        return this.tmpbmp3;
label_2498:
      bitmap6: Bitmap;
      return bitmap6;
    }

    pub void Se1_DrawAssetBlock(
      ref Graphics g,
      tx: i32,
      ty: i32,
      ref WindowClass tWindow,
      curAssetId: i32,
      assetRowOrSpecialCode: i32,
      specialOnX: i32,
      specialOnY: i32,
      specialType: i32,
      zoneNr: i32,
      zoneRegNr: i32,
      bool usePreviewMode = false)
    {
      libName1: String = "SE_Data";
      if (this.slotAssets < 1)
      {
        this.slotPaid = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 500, 0, 0));
        this.slotKeyReplace = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 515, 0, 0));
        this.slotHexNames = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 382, 0, 0));
        this.slotPerks = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 381, 0, 0));
        this.slotZones = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 123, 0, 0));
        this.slotAssets = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 148, 0, 0));
        this.slotAssetLog = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 241, 0, 0));
        this.slotAssetTypes = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 144, 0, 0));
        this.slotItemType = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 149, 0, 0));
        this.slotAssetPresentation = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 166, 0, 0));
        this.slotZones = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 123, 0, 0));
        this.slotDetailPreview = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 513, 0, 0));
        this.slotDetail = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 361, 0, 0));
        this.slotOrigDetail = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 361, 0, 0));
        this.slotConstruction = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 150, 0, 0));
      }
      this.slotDetail = !usePreviewMode ? this.slotOrigDetail : (this.slotDetailPreview <= -1 ? this.slotOrigDetail : this.slotDetailPreview);
      if (this.slotDetailPreview > this.game.Data.StringListObj.GetUpperBound(0))
        this.slotDetail = this.slotOrigDetail;
      let mut index1: i32 =  assetRowOrSpecialCode;
      let mut index2: i32 =  zoneRegNr;
      let mut id1: i32 =  this.game.Data.RegimeObj[index2].id;
      data1: DataClass = this.game.Data;
      str1: String = "perk";
      ref local1: String = ref str1;
      libName2: String = libName1;
      data1.FindLibVar(ref local1, libName2);
      data2: DataClass = this.game.Data;
      str1 = "hexname";
      ref local2: String = ref str1;
      libName3: String = libName1;
      let mut libVar: i32 =  data2.FindLibVar(ref local2, libName3);
      let mut idValue1: i32 =  -1;
      let mut num1: i32 =  0;
      x1: i32;
      y1: i32;
      idValue2: i32;
      idValue3: i32;
      num2: i32;
      idValue4: i32;
      num3: i32;
      regime1: i32;
      num4: i32;
      if (index1 >= 9000000 & index1 < 15000000)
      {
        idValue1 = specialType;
        x1 = specialOnX;
        y1 = specialOnY;
        idValue2 = -1;
        idValue3 = 9000000 + x1 * 1000 + y1;
        num2 = zoneNr;
        idValue4 = zoneNr;
        num3 = 0;
        regime1 = this.game.Data.MapObj[0].HexObj[x1, y1].Regime;
      }
      else if (index1 >= 15000000 & index1 < 16000000)
      {
        num1 = specialType;
        x1 = specialOnX;
        y1 = specialOnY;
        idValue2 = -1;
        idValue3 = 15000000 + x1 * 1000 + y1;
        num2 = zoneNr;
        idValue4 = zoneNr;
        num3 = 0;
        regime1 = this.game.Data.MapObj[0].HexObj[x1, y1].Regime;
      }
      else
      {
        idValue1 = -1;
        idValue3 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 9]));
        idValue2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 1]));
        num4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 3)));
        x1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 3]));
        y1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 4]));
        num3 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 5)));
        idValue4 = Conversions.ToInteger(this.game.EventRelatedObj.CheckLibVarHex(x1, y1, libName1, "Zones"));
        regime1 = this.game.Data.MapObj[0].HexObj[x1, y1].Regime;
        num2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 0]));
      }
      let mut regime2: i32 =  this.game.Data.MapObj[0].HexObj[x1, y1].Regime;
      let mut id2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotZones].GetData(0, zoneNr, 6)));
      let mut index3: i32 =  -1;
      let mut num5: i32 =  -1;
      let mut num6: i32 =  -1;
      if (id2 > 0)
      {
        index3 = this.game.HandyFunctionsObj.GetLocationByID(id2);
        if (index3 > -1)
        {
          num5 = this.game.Data.LocObj[index3].X;
          num6 = this.game.Data.LocObj[index3].Y;
        }
      }
      let mut num7: i32 =  0;
      let mut num8: i32 =  0;
      let mut num9: i32 =  0;
      let mut num10: i32 =  0;
      let mut val2_1: i32 =  0;
      let mut num11: i32 =  0;
      str2: String = "";
      str3: String = "";
      num12: i32;
      nr1: i32;
      str4: String;
      str5: String;
      num13: i32;
      str6: String;
      index4: i32;
      str7: String;
      str8: String;
      if (idValue2 > 0)
      {
        num7 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 25)));
        num8 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 5)));
        num9 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 13]));
        num10 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 11]));
        val2_1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 6]));
        num11 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 15]));
        num12 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 8]));
        nr1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 2)));
        str4 = this.game.Data.StringListObj[this.slotAssets].Data[index1, 10];
        if (this.game.Data.MapObj[0].HexObj[x1, y1].Location > -1 & idValue4 > 0 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotZones].GetData(0, idValue4, 6))) != this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[x1, y1].Location].ID)
        {
          str4 = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[x1, y1].Location].Name;
          this.game.Data.StringListObj[this.slotAssets].Data[index1, 10] = str4;
        }
        data3: String = this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 1);
        str5 = this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 12);
        num13 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 5]));
        str6 = num3 != 1 ? "Private Asset\r\n" : "Public Asset\r\n";
        let mut idValue5: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 0]));
        if (idValue5 != zoneNr)
          str6 = regime2 == this.game.Data.Turn ? str6 + "Belongs to zone: " + this.game.Data.StringListObj[this.slotZones].GetData(0, idValue5, 7) + ".\r\n" : str6 + "Is an evacuated asset. Cannot be used.\r\n";
        else if (index2 != this.game.Data.Turn & regime2 == this.game.Data.Turn)
          str6 += "Is an evacuated asset. Cannot be used.\r\n";
        if (num12 > 0)
          ;
        let mut num14: i32 =  0;
        if (!usePreviewMode)
        {
          let mut length: i32 =  this.game.Data.StringListObj[this.slotAssetLog].Length;
          for (let mut index5: i32 =  0; index5 <= length; index5 += 1)
          {
            if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetLog].Data[index5, 0])) == idValue3 && (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetLog].Data[index5, 1])) == 2 | (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetLog].Data[index5, 1])) == 1)
            {
              if (num14 == 0)
                str6 += "\r\nProblems or Special Modifiers:";
              num14 += 1;
              str6 = str6 + "\r\n• " + this.game.Data.StringListObj[this.slotAssetLog].Data[index5, 2];
            }
          }
        }
        if (!usePreviewMode && num12 > 0)
        {
          str6 += "\r\n\r\nConstruction costs paid so far:\r\n";
          let mut num15: i32 =  0;
          let mut length: i32 =  this.game.Data.StringListObj[this.slotPaid].Length;
          for (let mut index6: i32 =  0; index6 <= length; index6 += 1)
          {
            if (idValue3 == (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotPaid].Data[index6, 0])))
            {
              let mut num16: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotPaid].Data[index6, 1]));
              let mut num17: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotPaid].Data[index6, 3]));
              if (num17 > 0)
              {
                num15 += 1;
                num18: i32;
                switch (num16)
                {
                  case 1:
                    let mut num19: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotPaid].Data[index6, 2]));
                    num18 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotConstruction].GetData3(0, idValue2, 1, 2, 2, num19, 3)));
                    let mut num20: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 13)));
                    num18 *= num20;
                    str6 = str6 + "• " + num17.ToString() + " of " + num18.ToString() + " " + this.game.Data.StringListObj[this.slotItemType].GetData(0, num19, 1) + "\r\n";
                    continue;
                  case 2:
                    num18 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotConstruction].GetData3(0, idValue2, 1, 3, 2, this.game.Data.StringListObj[this.slotPaid].Data[index6, 2], 3)));
                    let mut num21: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 13)));
                    num18 *= num21;
                    if (Strings.InStr(this.game.Data.StringListObj[this.slotPaid].Data[index6, 2].ToLower(), "poppoints") > 0)
                    {
                      num17 *= 100;
                      num18 *= 100;
                    }
                    if (Strings.InStr(this.game.Data.StringListObj[this.slotPaid].Data[index6, 2].ToLower(), "workerpoints") > 0)
                    {
                      num17 *= 100;
                      num18 *= 100;
                    }
                    str6 = str6 + "• " + num17.ToString() + " of " + num18.ToString() + " " + this.game.Data.StringListObj[this.slotPaid].Data[index6, 2] + "\r\n";
                    continue;
                  default:
                    continue;
                }
              }
            }
          }
          if (num15 == 0)
            str6 += "• Nothing has been paid yet\r\n";
        }
        if (regime1 != this.game.Data.Turn)
          str6 = data3;
        index4 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 8)));
        str2 = data3;
        str3 = str6;
      }
      else if (idValue1 > 0)
      {
        index4 = Conversions.ToInteger(this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 13));
        nr1 = 0;
        str6 = "";
        str7 = "";
        str4 = "";
        str5 = this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 1);
        str2 = str5;
        str3 = this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 7) + "\r\n\r\n" + "A Hex Perk is completely independent from your Public and Private Economy and delivers as long as it is connected to the Zone City.";
      }
      else if (num1 > 0)
      {
        index4 = this.game.Data.FindEventPic(77, "SE_Asset");
        nr1 = num1 > 99 ? (num1 > 299 ? 3 : 2) : 1;
        str6 = "";
        str7 = "";
        str8 = "";
        str9: String = Conversions.ToString(num1 * 100);
        if (num1 * 100 >= 1000)
          str9 = Strings.Left(str9, str9.Length - 3) + "." + Strings.Right(str9, 3);
        str5 = str9 + " Free Folk";
        str2 = str5;
        str4 = this.game.Data.MapObj[0].HexObj[x1, y1].SmallLabel;
        str3 = "There is a settlement of " + (num1 * 100).ToString() + " Free Folk in Hex " + x1.ToString() + "," + y1.ToString() + ". Free Folk can reinforce your Population if Happiness is good.";
      }
      c2: Color;
      bitmap1: Bitmap;
      if (this.game.EditObj.se1_SelectAssetButton == idValue3)
      {
        if (num2 != zoneNr & idValue4 == zoneNr)
        {
          c2 = Color.FromArgb((int) byte.MaxValue, 195, 195, 225);
          ref Graphics local3 = ref g;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local4: Bitmap = ref bitmap2;
          let mut x2: i32 =  tx;
          let mut y2: i32 =  ty;
          double r =  ( c2.R /  byte.MaxValue) - 1.0;
          double g1 =  ( c2.G /  byte.MaxValue) - 1.0;
          double b =  ( c2.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local3, ref local4, x2, y2,  r,  g1,  b, 1f);
        }
        else if (num2 == zoneNr & idValue4 != zoneNr)
        {
          c2 = Color.FromArgb((int) byte.MaxValue, 235, 215, 235);
          ref Graphics local5 = ref g;
          bitmap3: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local6: Bitmap = ref bitmap3;
          let mut x3: i32 =  tx;
          let mut y3: i32 =  ty;
          double r =  ( c2.R /  byte.MaxValue) - 1.0;
          double g2 =  ( c2.G /  byte.MaxValue) - 1.0;
          double b =  ( c2.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local5, ref local6, x3, y3,  r,  g2,  b, 1f);
        }
        else if (idValue1 > 0)
        {
          c2 = Color.FromArgb((int) byte.MaxValue, 235, 235, 195);
          ref Graphics local7 = ref g;
          bitmap4: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local8: Bitmap = ref bitmap4;
          let mut x4: i32 =  tx;
          let mut y4: i32 =  ty;
          double r =  ( c2.R /  byte.MaxValue) - 1.0;
          double g3 =  ( c2.G /  byte.MaxValue) - 1.0;
          double b =  ( c2.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local7, ref local8, x4, y4,  r,  g3,  b, 1f);
        }
        else if (num1 > 0)
        {
          c2 = Color.FromArgb((int) byte.MaxValue, 215, 215, 175);
          ref Graphics local9 = ref g;
          bitmap5: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local10: Bitmap = ref bitmap5;
          let mut x5: i32 =  tx;
          let mut y5: i32 =  ty;
          double r =  ( c2.R /  byte.MaxValue) - 1.0;
          double g4 =  ( c2.G /  byte.MaxValue) - 1.0;
          double b =  ( c2.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local9, ref local10, x5, y5,  r,  g4,  b, 1f);
        }
        else if (num3 == 1)
        {
          ref Graphics local11 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local12: Bitmap = ref bitmap1;
          let mut x6: i32 =  tx;
          let mut y6: i32 =  ty;
          DrawMod.DrawSimple(ref local11, ref local12, x6, y6);
        }
        else
        {
          this.game.seColBrown = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 235, 215);
          ref Graphics local13 = ref g;
          bitmap6: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
          ref local14: Bitmap = ref bitmap6;
          let mut x7: i32 =  tx;
          let mut y7: i32 =  ty;
          double r =  ( this.game.seColBrown.R /  byte.MaxValue) - 1.0;
          double g5 =  ( this.game.seColBrown.G /  byte.MaxValue) - 1.0;
          double b =  ( this.game.seColBrown.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local13, ref local14, x7, y7,  r,  g5,  b, 1f);
        }
      }
      else if (num2 != zoneNr & idValue4 == zoneNr)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 195, 195, 225);
        ref Graphics local15 = ref g;
        bitmap7: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local16: Bitmap = ref bitmap7;
        let mut x8: i32 =  tx;
        let mut y8: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue - 1.1f);
        double g6 =  ( color.G /  byte.MaxValue - 1.1f);
        double b =  ( color.B /  byte.MaxValue - 1.1f);
        DrawMod.Draw(ref local15, ref local16, x8, y8,  r,  g6,  b, 1f);
      }
      else if (num2 == zoneNr & idValue4 != zoneNr)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 235, 215, 235);
        ref Graphics local17 = ref g;
        bitmap8: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local18: Bitmap = ref bitmap8;
        let mut x9: i32 =  tx;
        let mut y9: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue - 1.1f);
        double g7 =  ( color.G /  byte.MaxValue - 1.1f);
        double b =  ( color.B /  byte.MaxValue - 1.1f);
        DrawMod.Draw(ref local17, ref local18, x9, y9,  r,  g7,  b, 1f);
      }
      else if (idValue1 > 0)
      {
        c2 = Color.FromArgb((int) byte.MaxValue, 235, 235, 195);
        ref Graphics local19 = ref g;
        bitmap9: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local20: Bitmap = ref bitmap9;
        let mut x10: i32 =  tx;
        let mut y10: i32 =  ty;
        double r =  ( c2.R /  byte.MaxValue) - 1.0;
        double g8 =  ( c2.G /  byte.MaxValue) - 1.0;
        double b =  ( c2.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local19, ref local20, x10, y10,  r,  g8,  b, 1f);
      }
      else if (num1 > 0)
      {
        c2 = Color.FromArgb((int) byte.MaxValue, 215, 215, 175);
        ref Graphics local21 = ref g;
        bitmap10: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local22: Bitmap = ref bitmap10;
        let mut x11: i32 =  tx;
        let mut y11: i32 =  ty;
        double r =  ( c2.R /  byte.MaxValue) - 1.0;
        double g9 =  ( c2.G /  byte.MaxValue) - 1.0;
        double b =  ( c2.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local21, ref local22, x11, y11,  r,  g9,  b, 1f);
      }
      else if (num3 == 1)
      {
        ref Graphics local23 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local24: Bitmap = ref bitmap1;
        let mut x12: i32 =  tx;
        let mut y12: i32 =  ty;
        DrawMod.Draw(ref local23, ref local24, x12, y12, -0.1f, -0.1f, -0.1f, 1f);
      }
      else
      {
        this.game.seColBrown = Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, 235, 215);
        ref Graphics local25 = ref g;
        bitmap11: Bitmap = BitmapStore.GetBitmap(this.game.SE1_ASSETBACKGROUND);
        ref local26: Bitmap = ref bitmap11;
        let mut x13: i32 =  tx;
        let mut y13: i32 =  ty;
        double r =  ( this.game.seColBrown.R /  byte.MaxValue - 1.1f);
        double g10 =  ( this.game.seColBrown.G /  byte.MaxValue - 1.1f);
        double b =  ( this.game.seColBrown.B /  byte.MaxValue - 1.1f);
        DrawMod.Draw(ref local25, ref local26, x13, y13,  r,  g10,  b, 1f);
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (index4 > 0)
      {
        let mut num22: i32 =  nr1;
        if (num22 > 5)
          num22 = 5;
        if (num22 < 1)
          num22 = 1;
        let mut x14: i32 =  2 + (num22 - 1) * 134;
        ref Graphics local27 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index4]);
        ref local28: Bitmap = ref bitmap1;
        rectangle1 = Rectangle::new(x14, 2, 131, 111);
        let mut srcrect: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(tx + 12, ty + 40, 131, 111);
        let mut destrect: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2(ref local27, ref local28, srcrect, destrect);
      }
      if (this.game.EditObj.se1_SelectAssetButton != idValue3)
      {
        if (num2 != zoneNr & idValue4 == zoneNr)
        {
          color: Color = Color.FromArgb((int) byte.MaxValue, 195, 195, 225);
          ref Graphics local29 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local30: Bitmap = ref bitmap1;
          let mut x15: i32 =  tx;
          let mut y14: i32 =  ty;
          double r =  ( color.R /  byte.MaxValue) - 1.0;
          double g11 =  ( color.G /  byte.MaxValue) - 1.0;
          double b =  ( color.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local29, ref local30, x15, y14,  r,  g11,  b, 1f);
        }
        else if (num2 == zoneNr & idValue4 != zoneNr)
        {
          color: Color = Color.FromArgb((int) byte.MaxValue, 235, 215, 235);
          ref Graphics local31 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local32: Bitmap = ref bitmap1;
          let mut x16: i32 =  tx;
          let mut y15: i32 =  ty;
          double r =  ( color.R /  byte.MaxValue) - 1.0;
          double g12 =  ( color.G /  byte.MaxValue) - 1.0;
          double b =  ( color.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local31, ref local32, x16, y15,  r,  g12,  b, 1f);
        }
        else if (num3 == 1)
        {
          ref Graphics local33 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local34: Bitmap = ref bitmap1;
          let mut x17: i32 =  tx;
          let mut y16: i32 =  ty;
          DrawMod.DrawSimple(ref local33, ref local34, x17, y16);
        }
        else if (idValue1 > 0)
        {
          color: Color = Color.FromArgb((int) byte.MaxValue, 235, 235, 195);
          ref Graphics local35 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local36: Bitmap = ref bitmap1;
          let mut x18: i32 =  tx;
          let mut y17: i32 =  ty;
          double r =  ( color.R /  byte.MaxValue) - 1.0;
          double g13 =  ( color.G /  byte.MaxValue) - 1.0;
          double b =  ( color.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local35, ref local36, x18, y17,  r,  g13,  b, 1f);
        }
        else if (num1 > 0)
        {
          color: Color = Color.FromArgb((int) byte.MaxValue, 215, 215, 175);
          ref Graphics local37 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local38: Bitmap = ref bitmap1;
          let mut x19: i32 =  tx;
          let mut y18: i32 =  ty;
          double r =  ( color.R /  byte.MaxValue) - 1.0;
          double g14 =  ( color.G /  byte.MaxValue) - 1.0;
          double b =  ( color.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local37, ref local38, x19, y18,  r,  g14,  b, 1f);
        }
        else
        {
          this.game.seColBrown = Color.FromArgb((int) byte.MaxValue, 245, 225, 205);
          ref Graphics local39 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDER);
          ref local40: Bitmap = ref bitmap1;
          let mut x20: i32 =  tx;
          let mut y19: i32 =  ty;
          double r =  ( this.game.seColBrown.R /  byte.MaxValue) - 1.0;
          double g15 =  ( this.game.seColBrown.G /  byte.MaxValue) - 1.0;
          double b =  ( this.game.seColBrown.B /  byte.MaxValue) - 1.0;
          DrawMod.Draw(ref local39, ref local40, x20, y19,  r,  g15,  b, 1f);
        }
      }
      else if (num2 != zoneNr & idValue4 == zoneNr)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 215, 215, 235);
        ref Graphics local41 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local42: Bitmap = ref bitmap1;
        let mut x21: i32 =  tx;
        let mut y20: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue) - 1.0;
        double g16 =  ( color.G /  byte.MaxValue) - 1.0;
        double b =  ( color.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local41, ref local42, x21, y20,  r,  g16,  b, 1f);
      }
      else if (num2 == zoneNr & idValue4 != zoneNr)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 235, 215, 235);
        ref Graphics local43 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local44: Bitmap = ref bitmap1;
        let mut x22: i32 =  tx;
        let mut y21: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue) - 1.0;
        double g17 =  ( color.G /  byte.MaxValue) - 1.0;
        double b =  ( color.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local43, ref local44, x22, y21,  r,  g17,  b, 1f);
      }
      else if (idValue1 > 0)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 235, 235, 195);
        ref Graphics local45 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local46: Bitmap = ref bitmap1;
        let mut x23: i32 =  tx;
        let mut y22: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue) - 1.0;
        double g18 =  ( color.G /  byte.MaxValue) - 1.0;
        double b =  ( color.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local45, ref local46, x23, y22,  r,  g18,  b, 1f);
      }
      else if (num1 > 0)
      {
        color: Color = Color.FromArgb((int) byte.MaxValue, 215, 215, 175);
        ref Graphics local47 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local48: Bitmap = ref bitmap1;
        let mut x24: i32 =  tx;
        let mut y23: i32 =  ty;
        double r =  ( color.R /  byte.MaxValue) - 1.0;
        double g19 =  ( color.G /  byte.MaxValue) - 1.0;
        double b =  ( color.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local47, ref local48, x24, y23,  r,  g19,  b, 1f);
      }
      else if (num3 == 1)
      {
        ref Graphics local49 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local50: Bitmap = ref bitmap1;
        let mut x25: i32 =  tx;
        let mut y24: i32 =  ty;
        DrawMod.DrawSimple(ref local49, ref local50, x25, y24);
      }
      else
      {
        this.game.seColBrown = Color.FromArgb((int) byte.MaxValue, 245, 225, 205);
        ref Graphics local51 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERHIGH);
        ref local52: Bitmap = ref bitmap1;
        let mut x26: i32 =  tx;
        let mut y25: i32 =  ty;
        double r =  ( this.game.seColBrown.R /  byte.MaxValue) - 1.0;
        double g20 =  ( this.game.seColBrown.G /  byte.MaxValue) - 1.0;
        double b =  ( this.game.seColBrown.B /  byte.MaxValue) - 1.0;
        DrawMod.Draw(ref local51, ref local52, x26, y25,  r,  g20,  b, 1f);
      }
      if (!usePreviewMode && idValue2 > 0)
      {
        if (num9 > 0 & index2 == this.game.Data.Turn)
        {
          let mut h: i32 =  (int) Math.Round( (63 * num9) / 100.0);
          color: Color = DrawMod.LightenColor(this.game.seColYellow, num9 - 100);
          c2 = DrawMod.LightenColor(color, -100);
          DrawMod.DrawBlockGradient2(ref g, tx + 147, ty + 107 + (63 - h), 3, h, color, c2);
        }
        WindowClass windowClass1 = tWindow;
        rectangle2 = Rectangle::new(tx + 145, ty + 107, 7, 63);
        let mut rectangle3: &Rectangle = &rectangle2
        ref Rectangle local53 = ref rectangle3;
        ttitle1: String = "Upkeep: " + num9.ToString() + "%";
        windowClass1.AddMouse(ref local53, ttitle1, "...");
        if (num12 > 0)
        {
          if (num10 > 0 & index2 == this.game.Data.Turn)
          {
            let mut h: i32 =  (int) Math.Round( (63 * num10) / 100.0);
            color: Color = DrawMod.LightenColor(this.game.seColBlue, num10 - 100);
            c2 = DrawMod.LightenColor(color, -100);
            DrawMod.DrawBlockGradient2(ref g, tx + 147, ty + 34 + (63 - h), 3, h, color, c2);
          }
          WindowClass windowClass2 = tWindow;
          rectangle2 = Rectangle::new(tx + 145, ty + 34, 7, 63);
          let mut rectangle4: &Rectangle = &rectangle2
          ref Rectangle local54 = ref rectangle4;
          ttitle2: String = "Construction: " + num10.ToString() + "%";
          windowClass2.AddMouse(ref local54, ttitle2, "...");
        }
        else
        {
          if (num10 > 0 & index2 == this.game.Data.Turn)
          {
            let mut h: i32 =  (int) Math.Round( (63 * num10) / 100.0);
            if (num11 > 0)
              h = (int) Math.Round(63.0 * ( (num10 * num11) / 100.0) / 100.0);
            color: Color = DrawMod.LightenColor(this.game.seColGreen, num10 - 100);
            c2 = DrawMod.LightenColor(color, -100);
            DrawMod.DrawBlockGradient2(ref g, tx + 147, ty + 34 + (63 - h), 3, h, color, c2);
          }
          WindowClass windowClass3 = tWindow;
          rectangle2 = Rectangle::new(tx + 145, ty + 34, 7, 63);
          let mut rectangle5: &Rectangle = &rectangle2
          ref Rectangle local55 = ref rectangle5;
          ttitle3: String = "Production: " + num10.ToString() + "%";
          windowClass3.AddMouse(ref local55, ttitle3, "...");
        }
        if (val2_1 > 0)
        {
          let mut h: i32 =  (int) Math.Round( (137 * Math.Min(200 * nr1, val2_1)) /  (200 * nr1));
          color: Color = DrawMod.LightenColor(this.game.seColRed, -(int) Math.Round( Math.Min(200 * nr1, val2_1) /  (200 * nr1)));
          c2 = DrawMod.LightenColor(color, -100);
          DrawMod.DrawBlockGradient2(ref g, tx + 4, ty + 34 + (137 - h), 3, h, color, c2);
        }
        WindowClass windowClass4 = tWindow;
        rectangle2 = Rectangle::new(tx + 2, ty + 34, 7, 137);
        rectangle1 = rectangle2;
        ref Rectangle local56 = ref rectangle1;
        ttext: String = "Damage: " + val2_1.ToString() + " pts\r\nCompletely inoperable at " + (200 * nr1).ToString() + " points.\r\n100% certain loss of Asset at " + (1000 + 200 * nr1).ToString() + " points.";
        windowClass4.AddMouse(ref local56, "Damage to Asset", ttext);
      }
      str10: String = str5;
      if (str10.Length > 18)
        str10 = Strings.Left(str10, 18) + ".";
      if (idValue2 > 0 & nr1 > 0)
      {
        str11: String = str5;
        str10 = (str11.Length <= 16 ? str11 + " " : Strings.Left(str11, 16) + ".") + this.game.HandyFunctionsObj.GetRomanNumerical(nr1);
      }
      DrawMod.DrawTextColouredConsoleCenter(ref g, str10, DrawMod.TGame.MarcFont7, tx + 78, ty + 174, DrawMod.TGame.seColGray);
      if (num12 == 0 & idValue2 > 0 & (!this.game.Data.FOWOn | index2 == this.game.Data.Turn))
      {
        str7 = "";
        ref Graphics local57 = ref g;
        bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERCORNER);
        ref local58: Bitmap = ref bitmap1;
        rectangle2 = Rectangle::new(0, 0, 28, 27);
        let mut srcrect1: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(tx + 131, ty + 0, 28, 27);
        let mut destrect1: &Rectangle = &rectangle1
        double r =  ( this.game.seColBrown.R /  byte.MaxValue) - 1.0;
        double g21 =  ( this.game.seColBrown.G /  byte.MaxValue) - 1.0;
        double b =  ( this.game.seColBrown.B /  byte.MaxValue) - 1.0;
        DrawMod.DrawSimplePart2Coloured(ref local57, ref local58, srcrect1, destrect1,  r,  g21,  b, 1f);
        switch (num13)
        {
          case -2:
            str6 = "Mode: Closed";
            ref Graphics local59 = ref g;
            bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERCORNER);
            ref local60: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(84, 0, 28, 27);
            let mut srcrect2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 131, ty + 0, 28, 27);
            let mut destrect2: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local59, ref local60, srcrect2, destrect2);
            break;
          case -1:
            str6 = "Mode: Mothballed";
            ref Graphics local61 = ref g;
            bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERCORNER);
            ref local62: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(56, 0, 28, 27);
            let mut srcrect3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 131, ty + 0, 28, 27);
            let mut destrect3: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local61, ref local62, srcrect3, destrect3);
            break;
          default:
            str6 = "Mode: Active";
            ref Graphics local63 = ref g;
            bitmap1 = BitmapStore.GetBitmap(this.game.SE1_ASSETBORDERCORNER);
            ref local64: Bitmap = ref bitmap1;
            rectangle2 = Rectangle::new(28, 0, 28, 27);
            let mut srcrect4: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(tx + 131, ty + 0, 28, 27);
            let mut destrect4: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2(ref local63, ref local64, srcrect4, destrect4);
            break;
        }
        WindowClass windowClass = tWindow;
        rectangle2 = Rectangle::new(tx + 131, ty + 0, 28, 27);
        rectangle1 = rectangle2;
        ref Rectangle local65 = ref rectangle1;
        ttext: String = str6;
        windowClass.AddMouse(ref local65, "", ttext);
      }
      else if (idValue1 <= 0)
        ;
      str12: String = "";
      if (index3 > -1)
      {
        if (this.game.Data.LocObj[index3].X == x1 & this.game.Data.LocObj[index3].Y == y1)
          str12 = "City";
        else if (num4 >= 1 & str4.Length > 1)
          str12 = str4;
      }
      else if (num4 >= 1 & str4.Length > 1)
        str12 = str4;
      if (num2 != idValue4 & zoneNr == idValue4)
      {
        str12 = "DELEGATED";
        let mut smallPic: i32 =  this.game.Data.FindSmallPic(165, "SE_Graphics");
        if (smallPic > -1)
        {
          ref Graphics local66 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], 1);
          ref local67: Bitmap = ref bitmap1;
          let mut x27: i32 =  tx - 35;
          let mut y26: i32 =  ty - 3;
          DrawMod.DrawSimple(ref local66, ref local67, x27, y26);
        }
      }
      if (num2 != idValue4 & zoneNr == num2)
      {
        str12 = this.game.Data.StringListObj[this.slotZones].GetData(0, idValue4, 7);
        let mut smallPic: i32 =  this.game.Data.FindSmallPic(165, "SE_Graphics");
        if (smallPic > -1)
        {
          ref Graphics local68 = ref g;
          bitmap1 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[smallPic], 1);
          ref local69: Bitmap = ref bitmap1;
          let mut x28: i32 =  tx - 35;
          let mut y27: i32 =  ty - 3;
          DrawMod.DrawSimple(ref local68, ref local69, x28, y27);
        }
      }
      if (idValue1 > 0)
      {
        str12 = "HEX PERK";
        let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[x1, y1].GetHexLibVarValue(libVar);
        if (hexLibVarValue > 0)
        {
          str4 = this.game.Data.StringListObj[this.slotHexNames].GetData(0, hexLibVarValue, 1);
          if (str4.Length > 0)
            str12 = str4;
        }
      }
      if (num1 > 0 && num1 > 0)
        str12 = str4;
      if (str12.Length > 0)
      {
        upper: String = str12.ToUpper();
        if (num3 == 1)
          DrawMod.DrawTextColouredConsoleCenterEmbossed(ref g, upper, DrawMod.TGame.MarcFont7, tx + 78, ty + 2, Color.Gray);
        else
          DrawMod.DrawTextColouredConsoleCenterEmbossed(ref g, upper, DrawMod.TGame.MarcFont7, tx + 78, ty + 2, Color.FromArgb((int) byte.MaxValue, (int) Math.Round( this.game.seColBrown.R / 2.0), (int) Math.Round( this.game.seColBrown.G / 2.0), (int) Math.Round( this.game.seColBrown.B / 2.0)));
      }
      SimpleStringList simpleStringList1 = SimpleStringList::new();
      SimpleStringList simpleStringList2 = SimpleStringList::new();
      SimpleStringList simpleStringList3 = SimpleStringList::new();
      SimpleStringList simpleStringList4 = SimpleStringList::new();
      SimpleStringList simpleStringList5 = SimpleStringList::new();
      SimpleStringList simpleStringList6 = SimpleStringList::new();
      SimpleStringList simpleStringList7 = SimpleStringList::new();
      SimpleStringList simpleStringList8 = SimpleStringList::new();
      let mut num23: i32 =  tx;
      bool flag = false;
      tweight1: i32;
      if (idValue1 > 0 & (!this.game.Data.FOWOn | index2 == this.game.Data.Turn))
      {
        let mut num24: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 2)));
        data4: String = this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 3);
        let mut num25: i32 =  (int) Math.Round(Conversion.Val(data4));
        data5: String = this.game.Data.StringListObj[this.slotPerks].GetData(0, idValue1, 5);
        tweight1 = (int) Math.Round(Conversion.Val(data5));
        if (Operators.CompareString(tweight1.ToString(), data5, false) != 0)
          tweight1 = 0;
        tdata1: i32;
        if (num24 == 1)
        {
          str6 = data4;
          tdata1 = 3;
        }
        if (num24 == 3)
        {
          str6 = this.game.Data.StringListObj[this.slotItemType].GetData(0, num25, 1);
          tdata1 = 2;
        }
        if (num24 == 2)
        {
          str6 = data4;
          tdata1 = 1;
        }
        if (num24 >= 1 & num24 <= 3)
          simpleStringList6.AddWeight(str6, tweight1, tdata1, num25);
      }
      if (idValue2 > 0 & (!this.game.Data.FOWOn | index2 == this.game.Data.Turn))
      {
        let mut length1: i32 =  this.game.Data.StringListObj[this.slotDetail].Length;
        for (let mut index7: i32 =  0; index7 <= length1; index7 += 1)
        {
          if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index7, 0])) == idValue3)
          {
            let mut num26: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index7, 1]));
            let mut num27: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index7, 2]));
            let mut num28: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index7, 3]));
            str8 = this.game.Data.StringListObj[this.slotDetail].Data[index7, 3];
            let mut num29: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index7, 4]));
            if (num27 == 5)
              flag = true;
          }
        }
        let mut length2: i32 =  this.game.Data.StringListObj[this.slotDetail].Length;
        for (let mut index8: i32 =  0; index8 <= length2; index8 += 1)
        {
          if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index8, 0])) == idValue3)
          {
            let mut tdata1: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index8, 1]));
            let mut num30: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index8, 2]));
            let mut num31: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index8, 3]));
            str13: String = this.game.Data.StringListObj[this.slotDetail].Data[index8, 3];
            let mut tweight2: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotDetail].Data[index8, 4]));
            str6 = "";
            if (tdata1 == 1)
              str6 = str13;
            if (tdata1 == 2)
              str6 = this.game.Data.StringListObj[this.slotItemType].GetData(0, num31, 1);
            if (tdata1 == 3)
              str6 = str13;
            if (tdata1 == 4)
              str6 = "Construction";
            if (tdata1 == 6 | tdata1 == 5)
              str6 = str13;
            if (Operators.CompareString(str6, "", false) == 0)
              str6 = str6;
            if (str6.Length > 0)
              str6 = Strings.Left(str6, 1).ToUpper() + Strings.Mid(str6, 2);
            if (num30 == 1 & (num31 > 0 | str13.Length > 0))
            {
              simpleStringList3.AddWeight(str6, tweight2, tdata1, num31);
              simpleStringList1.AddWeight(str6, tweight2, tdata1, num31);
            }
            if ((num30 == 3 | num30 == 5) & tweight2 > 0)
            {
              simpleStringList2.AddWeight(str6, tweight2, tdata1);
              simpleStringList1.AddWeight(str6, tweight2, tdata1, num31);
            }
            if (num30 == 21)
            {
              let mut nr2: i32 =  simpleStringList8.FindNr(str6);
              if (tweight2 > 100)
                tweight2 = 100;
              if (nr2 < 0)
                simpleStringList8.AddWeight(str6, tweight2, tdata1, num31);
              else if (tweight2 < simpleStringList8.Weight[nr2])
                simpleStringList8.Weight[nr2] = tweight2;
            }
            if (num30 == 23 | num30 == 25)
            {
              let mut nr3: i32 =  simpleStringList7.FindNr(str6);
              if (tweight2 > 100)
                tweight2 = 100;
              if (nr3 < 0)
                simpleStringList7.AddWeight(str6, tweight2, tdata1, num31);
              else if (tweight2 < simpleStringList8.Weight[nr3])
                simpleStringList7.Weight[nr3] = tweight2;
            }
            if (num30 == 2)
              simpleStringList5.AddWeight(str6, tweight2, tdata1, num31);
            if (num30 == 4 | num30 == 6)
              simpleStringList4.AddWeight(str6, tweight2, tdata1, num31);
            if (num30 == 14 | num30 == 16)
              simpleStringList6.AddWeight(str6, tweight2, tdata1, num31);
          }
        }
        simpleStringList2.ReverseSort();
        simpleStringList4.ReverseSort();
        simpleStringList6.ReverseSort();
      }
      let mut num32: i32 =  ty;
      num33: i32;
      val2_2: i32;
      str14: String;
      double num34;
      if (simpleStringList1.Counter > -1)
      {
        let mut counter: i32 =  simpleStringList1.Counter;
        for (let mut index9: i32 =  0; index9 <= counter; index9 += 1)
        {
          if (index9 <= 3)
          {
            str15: String = simpleStringList1.Id[index9];
            num33 = simpleStringList2.FindWeightById(str15) + Math.Max(0, simpleStringList3.FindWeightById(str15));
            val2_2 = Math.Max(0, simpleStringList4.FindWeightById(simpleStringList1.Id[index9])) + simpleStringList5.FindWeightById(str15);
            let mut val1: i32 =  (int) Math.Round(Math.Floor( Math.Max(0, simpleStringList2.FindWeightById(str15)) * ( simpleStringList7.FindWeightById(str15) / 100.0))) + (int) Math.Round(Math.Floor( Math.Max(0, simpleStringList3.FindWeightById(str15)) * ( simpleStringList8.FindWeightById(str15) / 100.0)));
            if (val2_2 < 0)
              val2_2 = 0;
            if (val1 < 0)
              val1 = 0;
            if (val1 > num33)
              val1 = num33;
            if (val2_2 > 0 & val1 > 0 & Math.Abs(val2_2 - val1) <= 1)
              val1 = val2_2;
            str7 = val2_2.ToString() + "/" + num33.ToString();
            if (val1 > val2_2)
              str7 = val2_2.ToString() + "(" + val1.ToString() + ")/" + num33.ToString();
            if (str15.Length > 15)
              str14 = Strings.Left(str15, 15);
            Color.FromArgb((int) byte.MaxValue, 90, 90, 90);
            if (num33 < 1)
              num33 = 1;
            let mut num35: i32 =  (int) Math.Round( (100 * val1) /  num33);
            let mut num36: i32 =  (int) Math.Round( (100 * val2_2) /  num33);
            if (num36 > num10)
              num36 = num10;
            num37: i32;
            if (num35 > num36)
              num37 = (int) Math.Round( (188 * val1) /  num33);
            num37 = (int) Math.Round( (188 * Math.Max(val1, val2_2)) /  num33);
            let mut num38: i32 =  (int) Math.Round( (100 * Math.Max(val1, val2_2)) /  num33);
            c: Color = this.game.seColGray;
            if (num38 >= 100)
              c = this.game.seColGray;
            if (num38 >= 75 & num38 <= 99)
              c = this.game.seColYellow;
            if (num38 >= 50 & num38 <= 74)
              c = this.game.seColYellow;
            if (num38 <= 49)
              c = this.game.seColRed;
            let mut x29: i32 =  tx + 20;
            let mut y28: i32 =  ty + 25;
            if (index9 == 1 | index9 == 3)
              x29 = tx + 80;
            if (index9 >= 2)
              y28 = ty + 45;
            if (index9 == 0 & simpleStringList1.Counter == 0)
              x29 += 40;
            if (index9 == 2 & simpleStringList1.Counter == 2)
              x29 += 40;
            if (index9 == 4 & simpleStringList1.Counter == 4)
              x29 += 40;
            if (index9 <= 1 & simpleStringList1.Counter <= 1)
              y28 += 10;
            let mut index10: i32 =  -1;
            if (simpleStringList1.Data1[index9] == 1)
              index10 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, "", simpleStringList1.Id[index9]);
            if (simpleStringList1.Data1[index9] == 2)
              index10 = this.game.EventRelatedObj.GetEventPicSlotFor((int) Math.Round(Conversion.Val( simpleStringList1.Data2[index9])), "", "");
            if (simpleStringList1.Data1[index9] == 3)
              index10 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, simpleStringList1.Id[index9], "");
            str16: String = simpleStringList1.Id[index9];
            if (Operators.CompareString(str16.ToLower(), "poppoints", false) == 0)
            {
              str16 = "Population Points";
              num33 *= 100;
              val2_2 *= 100;
              val1 *= 100;
            }
            if (Operators.CompareString(str16.ToLower(), "workerpoints", false) == 0)
            {
              str16 = "Worker Points";
              num33 *= 100;
              val2_2 *= 100;
              val1 *= 100;
            }
            str17: String = "Needed " + num33.ToString() + " " + str16 + " and used " + val2_2.ToString() + " " + str16 + " of the " + val1.ToString() + " alloted to the Asset at start of turn.";
            WindowClass windowClass = tWindow;
            rectangle2 = Rectangle::new(x29, y28 - 3, 60, 20);
            rectangle1 = rectangle2;
            ref Rectangle local70 = ref rectangle1;
            ttitle: String = str16;
            ttext: String = str17;
            windowClass.AddMouse(ref local70, ttitle, ttext);
            if (index10 > -1)
            {
              ref Graphics local71 = ref g;
              bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index10]);
              ref local72: Bitmap = ref bitmap1;
              let mut x30: i32 =  x29;
              let mut y29: i32 =  y28;
              DrawMod.DrawSimple(ref local71, ref local72, x30, y29);
              x29 += 30;
            }
            if (val2_2 >= 1000)
            {
              num34 = Math.Round( val2_2 / 1000.0, 1);
              str6 = num34.ToString() + "K";
            }
            else
              str6 = val2_2.ToString();
            DrawMod.DrawTextColouredConsole(ref g, str6, this.game.MarcFont7, x29, y28, c);
          }
        }
      }
      if (simpleStringList6.Counter > -1)
      {
        ty = num32;
        let mut counter: i32 =  simpleStringList6.Counter;
        for (let mut index11: i32 =  0; index11 <= counter; index11 += 1)
        {
          if (index11 <= 4)
          {
            str18: String = simpleStringList6.Id[index11];
            if (str18.Length > 17)
              str14 = Strings.Left(str18, 17);
            num33 = simpleStringList6.Weight[index11];
            str7 = num33.ToString();
            let mut x31: i32 =  tx + 20;
            if (index11 == 1 | index11 == 3)
              x31 = tx + 80;
            y30: i32;
            if (simpleStringList6.Counter >= 4)
            {
              y30 = ty + 121;
              if (index11 == 2 | index11 == 3)
                y30 = ty + 137;
              if (index11 == 4 | index11 == 5)
                y30 = ty + 155;
            }
            else
            {
              y30 = ty + 130;
              if (index11 == 2 | index11 == 3)
                y30 = ty + 150;
            }
            if (index11 == 0 & simpleStringList6.Counter == 0)
              x31 += 40;
            if (index11 == 2 & simpleStringList6.Counter == 2)
              x31 += 40;
            if (index11 == 4 & simpleStringList6.Counter == 4)
              x31 += 40;
            if (index11 <= 1 & simpleStringList6.Counter <= 1)
              y30 += 10;
            if (index11 <= 1 & simpleStringList6.Counter <= 1 & idValue1 > 0)
              y30 += 5;
            let mut index12: i32 =  -1;
            seColGray: Color = this.game.seColGray;
            if (simpleStringList6.Data1[index11] == 1)
              index12 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, "", simpleStringList6.Id[index11]);
            if (simpleStringList6.Data1[index11] == 2)
              index12 = this.game.EventRelatedObj.GetEventPicSlotFor((int) Math.Round(Conversion.Val( simpleStringList6.Data2[index11])), "", "");
            if (simpleStringList6.Data1[index11] == 3)
              index12 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, simpleStringList6.Id[index11], "");
            if (simpleStringList6.Data1[index11] == 5)
              index12 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, simpleStringList6.Id[index11], "");
            if (simpleStringList6.Data1[index11] == 6)
              index12 = this.game.EventRelatedObj.GetEventPicSlotFor(-1, simpleStringList6.Id[index11], "");
            if (Operators.CompareString(simpleStringList6.Id[index11].ToLower(), "pop", false) == 0)
              num33 *= 100;
            if (Operators.CompareString(simpleStringList6.Id[index11].ToLower(), "worker", false) == 0)
              num33 *= 100;
            if (simpleStringList6.Data2[index11] == 9 | simpleStringList6.Data2[index11] == 12)
              num33 *= 100;
            if (Operators.CompareString(simpleStringList6.Id[index11].ToLower(), "construction", false) == 0)
            {
              if (simpleStringList6.Counter == 0)
              {
                if (usePreviewMode)
                {
                  DrawMod.DrawTextColouredConsoleCenter(ref g, "Under Construction", this.game.MarcFont7, tx + 80, y30 + 8, this.game.seColBlue);
                }
                else
                {
                  str6 = simpleStringList6.Id[index11];
                  Left: String = ( Math.Round( ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 7])) * 100 - (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 12]))) / 100.0, 1)).ToString();
                  if (Operators.CompareString(Left, "0", false) == 0 & num12 > 0)
                    Left = "0.1";
                  strArray1: Vec<String> = new string[5]
                  {
                    "Did ",
                    null,
                    null,
                    null,
                    null
                  };
                  strArray2: Vec<String> = strArray1;
                  num34 = Math.Round( num33 / 100.0, 2);
                  str19: String = num34.ToString();
                  strArray2[1] = str19;
                  strArray1[2] = " turn of construction at start of turn.\r\nStill to do: ";
                  strArray1[3] = Left;
                  strArray1[4] = " turns.";
                  str20: String = string.Concat(strArray1);
                  WindowClass windowClass = tWindow;
                  rectangle2 = Rectangle::new(x31, y30 - 11, 60, 40);
                  rectangle1 = rectangle2;
                  ref Rectangle local73 = ref rectangle1;
                  ttitle: String = str6;
                  ttext: String = str20;
                  windowClass.AddMouse(ref local73, ttitle, ttext);
                  if (index12 > -1)
                  {
                    ref Graphics local74 = ref g;
                    bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index12]);
                    ref local75: Bitmap = ref bitmap1;
                    let mut x32: i32 =  x31;
                    let mut y31: i32 =  y30;
                    DrawMod.DrawSimple(ref local74, ref local75, x32, y31);
                    num23 = x31 + 30;
                  }
                  ref Graphics local76 = ref g;
                  num34 = Math.Round( num33 / 100.0, 2);
                  tstring: String = num34.ToString() + "t";
                  marcFont7: Font = this.game.MarcFont7;
                  let mut x33: i32 =  tx + 80;
                  let mut y32: i32 =  y30 - 8;
                  c: Color = seColGray;
                  DrawMod.DrawTextColouredConsoleCenter(ref local76, tstring, marcFont7, x33, y32, c);
                  if (Operators.CompareString(Left, "0", false) == 0 & num12 < 1)
                    DrawMod.DrawTextColouredConsoleCenter(ref g, "(constr finished)", this.game.MarcFont7, tx + 80, y30 + 8, this.game.seColBlue);
                  else
                    DrawMod.DrawTextColouredConsoleCenter(ref g, "(" + Left + "t left)", this.game.MarcFont7, tx + 80, y30 + 8, this.game.seColBlue);
                }
              }
            }
            else
            {
              str6 = simpleStringList6.Id[index11];
              if (this.slotKeyReplace > -1)
              {
                let mut row: i32 =  this.game.Data.StringListObj[this.slotKeyReplace].FindRow(0, str6);
                if (row > -1)
                  str6 = this.game.Data.StringListObj[this.slotKeyReplace].Data[row, 1];
              }
              str21: String = "Produced " + num33.ToString() + " " + str6 + " at start of turn.";
              WindowClass windowClass = tWindow;
              rectangle2 = Rectangle::new(x31, y30 - 3, 60, 20);
              rectangle1 = rectangle2;
              ref Rectangle local77 = ref rectangle1;
              ttitle: String = str6;
              ttext: String = str21;
              windowClass.AddMouse(ref local77, ttitle, ttext);
              if (index12 > -1)
              {
                ref Graphics local78 = ref g;
                bitmap1 = BitmapStore.GetBitmap(this.game.Data.EventPicNr[index12]);
                ref local79: Bitmap = ref bitmap1;
                let mut x34: i32 =  x31;
                let mut y33: i32 =  y30;
                DrawMod.DrawSimple(ref local78, ref local79, x34, y33);
                x31 += 30;
              }
              DrawMod.DrawTextColouredConsole(ref g, num33.ToString(), this.game.MarcFont7, x31, y30, seColGray);
            }
          }
        }
        if ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue2, 4))) == 5)
        {
          val2_2 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].GetData(9, idValue3, 3)));
          let mut index13: i32 =  (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].GetData(9, idValue3, 4)));
          tweight1 = (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].GetData(9, idValue3, 11)));
          num33 = (int) Math.Round( (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[val2_2, index13].Location].Type].Logistical * tweight1) / 100.0);
          if (num33 > 0)
          {
            str14 = "Log. Extension";
            str7 = num33.ToString();
          }
        }
      }
      else if (num12 > 0 & index1 < 9000000 & (simpleStringList6.Counter > -1 | simpleStringList1.Counter > -1))
      {
        let mut num39: i32 =  ty + 121;
        Left: String = ( Math.Round( ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 7])) * 100 - (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 12]))) / 100.0, 1)).ToString();
        if (Operators.CompareString(Left, "0", false) == 0 & num12 < 1)
          DrawMod.DrawTextColouredConsoleCenter(ref g, "(constr finished)", this.game.MarcFont7, tx + 80, num39 + 8, this.game.seColBlue);
        else if (usePreviewMode)
          DrawMod.DrawTextColouredConsoleCenter(ref g, "Under Construction", this.game.MarcFont7, tx + 80, num39 + 8, this.game.seColBlue);
        else
          DrawMod.DrawTextColouredConsoleCenter(ref g, "(" + Left + "t left)", this.game.MarcFont7, tx + 80, num39 + 8, this.game.seColBlue);
      }
      if (index1 < 9000000 && simpleStringList6.Counter == -1 & simpleStringList1.Counter == -1 & num12 == 1)
      {
        let mut x35: i32 =  tx + 20;
        let mut y34: i32 =  ty + 134;
        Left: String = ( Math.Round( ((int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 7])) * 100 - (int) Math.Round(Conversion.Val(this.game.Data.StringListObj[this.slotAssets].Data[index1, 12]))) / 100.0, 1)).ToString();
        if (Operators.CompareString(Left, "0", false) == 0 & num12 > 0)
          Left = "0.1";
        str22: String = "Under Construction. Still to do: " + Left + " turns.";
        WindowClass windowClass = tWindow;
        rectangle2 = Rectangle::new(x35, y34 - 11, 60, 40);
        rectangle1 = rectangle2;
        ref Rectangle local80 = ref rectangle1;
        ttitle: String = str6;
        ttext: String = str22;
        windowClass.AddMouse(ref local80, ttitle, ttext);
        DrawMod.DrawTextColouredConsoleCenter(ref g, "Under Construction", this.game.MarcFont7, tx + 80, y34, this.game.seColBlue);
        DrawMod.DrawTextColouredConsoleCenter(ref g, Left + "t left", this.game.MarcFont7, tx + 80, y34 + 16, this.game.seColBlue);
      }
      if (curAssetId == idValue3)
      {
        if (idValue2 > 0)
        {
          WindowClass windowClass = tWindow;
          rectangle2 = Rectangle::new(tx, ty, 156, 208);
          rectangle1 = rectangle2;
          ref Rectangle local81 = ref rectangle1;
          ttitle: String = str2;
          ttext: String = str3 + "\r\nClick to for more information on this Asset Type.";
          let mut tdata2: i32 =  idValue3;
          windowClass.AddMouse(ref local81, ttitle, ttext, 121, tdata2);
        }
        else if (idValue1 > 0)
        {
          WindowClass windowClass = tWindow;
          rectangle2 = Rectangle::new(tx, ty, 156, 208);
          rectangle1 = rectangle2;
          ref Rectangle local82 = ref rectangle1;
          ttitle: String = str2;
          ttext: String = str3 + "\r\nClick to for more information on this Hex Perk.";
          let mut tdata2: i32 =  idValue3;
          windowClass.AddMouse(ref local82, ttitle, ttext, 121, tdata2);
        }
        else if (num1 > 0)
        {
          WindowClass windowClass = tWindow;
          rectangle2 = Rectangle::new(tx, ty, 156, 208);
          rectangle1 = rectangle2;
          ref Rectangle local83 = ref rectangle1;
          ttitle: String = str2;
          ttext: String = str3 + "\r\nClick to for more information on this Free Folk settlement.";
          let mut tdata2: i32 =  idValue3;
          windowClass.AddMouse(ref local83, ttitle, ttext, 121, tdata2);
        }
        else
        {
          WindowClass windowClass = tWindow;
          rectangle2 = Rectangle::new(tx, ty, 156, 208);
          rectangle1 = rectangle2;
          ref Rectangle local84 = ref rectangle1;
          ttitle: String = str2;
          ttext: String = str3 + "\r\nClick to for more information on this Asset Type.";
          let mut tdata2: i32 =  idValue3;
          windowClass.AddMouse(ref local84, ttitle, ttext, 121, tdata2);
        }
      }
      else
      {
        WindowClass windowClass = tWindow;
        rectangle2 = Rectangle::new(tx, ty, 156, 208);
        rectangle1 = rectangle2;
        ref Rectangle local85 = ref rectangle1;
        ttitle: String = str2;
        ttext: String = str3 + "\r\nClick to select.";
        let mut tdata2: i32 =  idValue3;
        windowClass.AddMouse(ref local85, ttitle, ttext, 121, tdata2);
      }
    }
  }
}
