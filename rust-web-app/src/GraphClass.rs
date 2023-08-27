// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.GraphClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class GraphClass : SubPartClass
  {
     Width: i32;
     Height: i32;
     game: GameClass;
     backbitmap: Bitmap;
     bx: i32;
     by: i32;
     Mat: Vec<i32>;
     MatTitle: String;
     Multiplier: i32;
     Color[] RegCol;
     RegName: Vec<String>;
     RegCount: i32;
     bool showNumbers;
     bool checkForRegime;
     tmin: i32;
     bool tminRun;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub GraphClass(
      tgame: GameClass,
      tMat: Vec<i32>,
      bool tshowNumbers,
      tMatTitle: String,
      tMultiplier: i32,
      Color[] tRegCol,
      tRegName: Vec<String>,
      tregcount: i32,
      twidth: i32,
      theight: i32,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      bool tcheckforregime = true)
      : base(twidth, theight)
    {
      this.Width = twidth;
      this.Height = theight;
      this.game = tgame;
      this.Mat = tMat;
      this.tminRun = false;
      this.checkForRegime = tcheckforregime;
      this.showNumbers = tshowNumbers;
      this.RegCol = tRegCol;
      this.RegName = tRegName;
      this.RegCount = tregcount;
      this.MatTitle = tMatTitle;
      this.Multiplier = tMultiplier;
      if (!Information.IsNothing( tbackbitmap))
      {
        this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) this.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      this.bx = bbx;
      this.by = bby;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Rectangle[] rectangleArray = Rectangle::new[10000];
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        graphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( graphics,  this.backbitmap, 0, 0);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      DrawMod.DrawBlockGradient2( graphics, 0, 0, this.Width - 1, this.Height - 1, this.game.MarcCol1, this.game.MarcCol2);
      numArray1: Vec<i32> = new int[this.Mat.GetUpperBound(0) + 1, this.Mat.GetUpperBound(1) + 1];
      numArray2: Vec<i32> = new int[this.Mat.GetUpperBound(0) + 1, this.Mat.GetUpperBound(1) + 1];
      let mut num1: i32 =  this.Width - 70;
      let mut num2: i32 =  60;
      let mut num3: i32 =   Math.Round( this.Height - (50.0 + Math.Max(2.0,  this.RegCount / 2.0 + 0.5) * 15.0));
      let mut num4: i32 =   Math.Round(20.0 + Math.Max(2.0,  this.RegCount / 2.0 + 0.5) * 15.0);
      let mut num5: i32 =   Math.Round(Conversion.Int( num1 /  Math.Max(10, this.game.Data.Round)));
      if (!this.tminRun)
      {
        this.tmin = 0;
        this.tminRun = true;
        let mut round1: i32 =  this.game.Data.Round;
        for (let mut index1: i32 =  1; index1 <= round1; index1 += 1)
        {
          let mut regCount: i32 =  this.RegCount;
          for (let mut index2: i32 =  1; index2 <= regCount; index2 += 1)
          {
            if (this.Mat[index1, index2] < this.tmin)
              this.tmin = this.Mat[index1, index2];
          }
        }
        let mut round2: i32 =  this.game.Data.Round;
        for (let mut index3: i32 =  1; index3 <= round2; index3 += 1)
        {
          let mut regCount: i32 =  this.RegCount;
          for (let mut index4: i32 =  1; index4 <= regCount; index4 += 1)
          {
            mat: Vec<i32> = this.Mat;
            numArray3: Vec<i32> = mat;
            let mut index5: i32 =  index3;
            let mut index6: i32 =  index5;
            let mut index7: i32 =  index4;
            let mut index8: i32 =  index7;
            let mut num6: i32 =  mat[index5, index7] + Math.Abs(this.tmin);
            numArray3[index6, index8] = num6;
          }
        }
      }
      let mut num7: i32 =  0;
      let mut round3: i32 =  this.game.Data.Round;
      for (let mut index9: i32 =  1; index9 <= round3; index9 += 1)
      {
        let mut regCount: i32 =  this.RegCount;
        for (let mut index10: i32 =  1; index10 <= regCount; index10 += 1)
        {
          if (this.Mat[index9, index10] > num7)
            num7 = this.Mat[index9, index10];
        }
      }
      let mut num8: i32 =  0;
      float a1 = 1E+09f;
      if (num7 < 10)
        num7 = 10;
label_43:
      if (num8 == 0)
      {
        a1 /= 10f;
        let mut num9: i32 =  1;
        do
        {
          num10: i32;
          if (num9 == 1)
            num10 =  Math.Round( a1);
          if (num9 == 2)
            num10 =  Math.Round( a1 * 0.8);
          if (num9 == 3)
            num10 =  Math.Round( a1 * 0.6);
          if (num9 == 4)
            num10 =  Math.Round( (a1 / 2f));
          if (num9 == 5)
            num10 =  Math.Round( a1 * 0.4);
          if (num9 >= 6)
            num10 =  Math.Round( a1 * 0.2);
          if (num10 > num7 & num10 <= num7 * 2 |  a1 < 10.0)
          {
            num7 = num10;
            goto label_44;
          }
          else
            num9 += 1;
        }
        while (num9 <= 8);
        goto label_43;
      }
label_44:
      let mut round4: i32 =  this.game.Data.Round;
      for (let mut index11: i32 =  1; index11 <= round4; index11 += 1)
      {
        let mut regCount1: i32 =  this.RegCount;
        for (let mut index12: i32 =  1; index12 <= regCount1; index12 += 1)
        {
          numArray1[index11, index12] =  Math.Round( Conversion.Int(num5 * index11) - Conversion.Int( num5 / 2.0));
          numArray2[index11, index12] =  Math.Round(Conversion.Int( this.Mat[index11, index12] /  num7 *  num3));
        }
        let mut num11: i32 =  1;
        while (num11 == 1)
        {
          num11 = 0;
          let mut regCount2: i32 =  this.RegCount;
          for (let mut index13: i32 =  1; index13 <= regCount2; index13 += 1)
          {
            let mut regCount3: i32 =  this.RegCount;
            for (let mut index14: i32 =  1; index14 <= regCount3; index14 += 1)
            {
              if (index13 != index14 && numArray2[index11, index13] > numArray2[index11, index14] - 1 & numArray2[index11, index13] < numArray2[index11, index14] + 1 && numArray2[index11, index13] >= numArray2[index11, index14])
              {
                numArray2[index11, index13] = numArray2[index11, index14] + 2;
                num11 = 1;
              }
            }
          }
        }
        let mut regCount4: i32 =  this.RegCount;
        for (let mut index15: i32 =  1; index15 <= regCount4; index15 += 1)
        {
          numArray2[index11, index15] = num4 + (num3 - numArray2[index11, index15]);
          numArray1[index11, index15] = num2 + numArray1[index11, index15];
        }
      }
      c1: Color = Color.FromArgb(128, 200, 200, 200);
      c2: Color = Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      DrawMod.drawLine( graphics, num2, num4 - 10, num2, this.Height - 5, c1);
      DrawMod.drawLine( graphics, 5, num4 + num3 + 10, this.Width - 5, num4 + num3 + 10, c1);
      let mut num12: i32 =  Math.Max(10, this.game.Data.Round);
      for (let mut index: i32 =  1; index <= num12; index += 1)
        DrawMod.drawLineDot( graphics, num2 + num5 * index, num4 - 10, num2 + num5 * index, this.Height - 5, c1);
      let mut num13: i32 =  Math.Max(10, this.game.Data.Round);
      SizeF sizeF2;
      for (let mut round5: i32 =  1; round5 <= num13; round5 += 1)
      {
        str: String = this.DateString(round5);
        sizeF2 = graphics.MeasureString(str, this.game.MarcFont5);
        float num14 =  num5;
        let mut num15: i32 =  1;
        for (;  num14 <  sizeF2.Width + 10.0; num14 =  (num5 * num15))
          num15 += 1;
        if (round5 % num15 == 0)
          DrawMod.DrawTextColouredMarc( graphics, str, this.game.MarcFont5,  Math.Round( (num2 + num5 * round5) -  num5 / 2.0 -  sizeF2.Width / 2.0), num4 + num3 + 12, c2);
      }
      let mut num16: i32 =  0;
      do
      {
        float num17 =  ( Math.Round( num7 / 10.0) * num16);
        let mut num18: i32 =   Math.Round( ( (num4 + num3) -  num3 * (num17 /  num7)));
        DrawMod.drawLineDot( graphics, num2 - 20, num18, num2 + num1, num18, c1);
        str: String = Strings.Trim(Conversion.Str( (( Math.Round( num7 / 10.0) * num16 - Math.Abs(this.tmin)) * this.Multiplier)));
        if (Operators.CompareString(Strings.Right(str, 4), "0000", false) == 0)
          str = Strings.Left(str, Strings.Len(str) - 3) + "K".to_owned();
        sizeF2 = graphics.MeasureString(str, this.game.MarcFont5);
        DrawMod.DrawTextColouredMarc( graphics, str, this.game.MarcFont5,  Math.Round( ( (num2 - 25) - sizeF2.Width / 2f)),  Math.Round( ( num18 - sizeF2.Height)), c2);
        num16 += 1;
      }
      while (num16 <= 10);
      DrawMod.DrawTextColouredMarc( graphics, this.MatTitle, this.game.MarcFont4, 20, 5, c2);
      sizeF2 = graphics.MeasureString(this.MatTitle, this.game.MarcFont4);
      let mut num19: i32 =   Math.Round( Math.Max(0.0f, 220f - sizeF2.Width));
      let mut y1_1: i32 =  -5;
      let mut x1_1: i32 =   Math.Round( ( num2 + sizeF2.Width));
      let mut num20: i32 =  0;
      let mut regCount5: i32 =  this.RegCount;
      for (let mut index: i32 =  1; index <= regCount5; index += 1)
      {
        if ( index > Math.Max(2.0, Conversion.Int( this.RegCount / 2.0 + 0.5)) & num20 == 0)
        {
          num20 = 1;
          y1_1 = -5;
          x1_1 =  Math.Round( num2 +  num1 / 2.0 +  num1 / 5.0 -  num19) + 75;
        }
        y1_1 += 15;
        color: Color = DrawMod.LightenColor(Color.FromArgb( this.RegCol[index].A,  this.RegCol[index].R,  this.RegCol[index].G,  this.RegCol[index].B), 66);
        DrawMod.DrawBlock( graphics, x1_1, y1_1, 20, 10,  this.RegCol[index].R,  this.RegCol[index].G,  this.RegCol[index].B,  this.RegCol[index].A);
        DrawMod.DrawRectangle( graphics, x1_1, y1_1, 20, 10,  color.R,  color.G,  color.B,  color.A);
        DrawMod.DrawRectangle( graphics, x1_1 - 1, y1_1 - 1, 22, 12, 0, 0, 0, 200);
        DrawMod.DrawTextColouredMarc( graphics, this.RegName[index], this.game.MarcFont5, x1_1 + 30, y1_1 - 2, c2);
      }
      let mut num21: i32 =  this.game.Data.Round - 1;
      for (let mut index16: i32 =  1; index16 <= num21; index16 += 1)
      {
        let mut regCount6: i32 =  this.RegCount;
        for (let mut index17: i32 =  1; index17 <= regCount6; index17 += 1)
        {
          if (!this.checkForRegime | !(this.game.Data.Round == index16 + 1 & this.game.Data.Turn < index17 - 1))
          {
            let mut num22: i32 =  numArray1[index16, index17];
            let mut num23: i32 =  numArray2[index16, index17];
            let mut x1_2: i32 =  num22;
            let mut num24: i32 =  num23;
            float a2 =  numArray1[index16 + 1, index17];
            let mut num25: i32 =  numArray2[index16 + 1, index17];
            let mut y1_2: i32 =  num24 + 0;
            let mut y2: i32 =  num25 + 0;
            color: Color = DrawMod.LightenColor(Color.FromArgb( this.RegCol[index17].A,  this.RegCol[index17].R,  this.RegCol[index17].G,  this.RegCol[index17].B), 66);
            DrawMod.drawLine( graphics, x1_2, y1_2 + 1,  Math.Round( a2), y2 + 1,  this.RegCol[index17].R,  this.RegCol[index17].G,  this.RegCol[index17].B,  this.RegCol[index17].A);
            DrawMod.drawLine( graphics, x1_2, y1_2,  Math.Round( a2), y2,  color.R,  color.G,  color.B,  color.A);
          }
        }
      }
      let mut round6: i32 =  this.game.Data.Round;
      for (let mut index18: i32 =  1; index18 <= round6; index18 += 1)
      {
        let mut regCount7: i32 =  this.RegCount;
        for (let mut index19: i32 =  1; index19 <= regCount7; index19 += 1)
        {
          if (!this.checkForRegime | !(this.game.Data.Round == index18 & this.game.Data.Turn < index19 - 1))
          {
            let mut num26: i32 =  numArray1[index18, index19];
            let mut num27: i32 =  numArray2[index18, index19];
            let mut num28: i32 =  num26 - 2;
            let mut num29: i32 =  num27 - 2;
            DrawMod.LightenColor(Color.FromArgb( this.RegCol[index19].A,  this.RegCol[index19].R,  this.RegCol[index19].G,  this.RegCol[index19].B), 66);
            if (this.showNumbers)
            {
              float num30 =  ((this.Mat[index18, index19] - Math.Abs(this.tmin)) * this.Multiplier);
              str: String = !( num30 > 9999.0 |  num30 < -9999.0) ? num30.ToString() : Strings.Format(  ( num30 / 1000.0), "0.0") + "K".to_owned();
              sizeF2 = graphics.MeasureString(str, this.game.MarcFont5);
              let mut num31: i32 =  0;
              index20: i32;
              let mut num32: i32 =  index20;
              for (let mut index21: i32 =  1; index21 <= num32; index21 += 1)
              {
                if (rectangleArray[index21].X >= num28 &  rectangleArray[index21].X <=  num28 +  sizeF2.Width && rectangleArray[index21].Y >= num29 &  rectangleArray[index21].Y <=  num29 +  sizeF2.Height)
                  num31 = 1;
                if (rectangleArray[index21].X >= num28 &  rectangleArray[index21].X <=  num28 +  sizeF2.Width && rectangleArray[index21].Y + rectangleArray[index21].Height >= num29 &  (rectangleArray[index21].Y + rectangleArray[index21].Height) <=  num29 +  sizeF2.Height)
                  num31 = 1;
                if (rectangleArray[index21].X + rectangleArray[index21].Width >= num28 &  (rectangleArray[index21].X + rectangleArray[index21].Width) <=  num28 +  sizeF2.Width && rectangleArray[index21].Y >= num29 &  rectangleArray[index21].Y <=  num29 +  sizeF2.Height)
                  num31 = 1;
                if (rectangleArray[index21].X + rectangleArray[index21].Width >= num28 &  (rectangleArray[index21].X + rectangleArray[index21].Width) <=  num28 +  sizeF2.Width && rectangleArray[index21].Y + rectangleArray[index21].Height >= num29 &  rectangleArray[index21].Y <=  num29 +  sizeF2.Height)
                  num31 = 1;
              }
              if (num31 == 0)
              {
                index20 += 1;
                c3: Color = DrawMod.LightenColor(Color.FromArgb( this.RegCol[index19].A,  this.RegCol[index19].R,  this.RegCol[index19].G,  this.RegCol[index19].B), 66);
                DrawMod.DrawBlock( graphics, num28, num29, 5, 5,  this.RegCol[index19].R,  this.RegCol[index19].G,  this.RegCol[index19].B,  this.RegCol[index19].A);
                DrawMod.DrawRectangle( graphics, num28, num29, 5, 5,  c3.R,  c3.G,  c3.B,  c3.A);
                DrawMod.DrawRectangle( graphics, num28 - 1, num29 - 1, 7, 7, 0, 0, 0, 80);
                DrawMod.DrawBlock( graphics, num28 + 6, num29 - 2,  Math.Round( sizeF2.Width),  Math.Round( (sizeF2.Height - 3f)), 0, 0, 0, 100);
                rectangleArray[index20] = this.game.Data.Product < 6 ? Rectangle::new(num28, num29,  Math.Round( sizeF2.Width),  Math.Round( sizeF2.Height)) : Rectangle::new(num28 - 8, num29 - 4,  Math.Round( (sizeF2.Width + 16f)),  Math.Round( (sizeF2.Height + 8f)));
                DrawMod.DrawTextColouredMarc( graphics, str, this.game.MarcFont5, num28 + 6, num29 - 3, c3);
              }
            }
          }
        }
      }
      DrawMod.DrawFrame( this.OwnBitmap,  this.backbitmap,  graphics, 0, 0, this.Width, this.Height, 0, 0);
      return this.OwnBitmap;
    }

    pub DateString: String(round: i32)
    {
      object Counter;
      str: String;
      if (this.game.Data.AlternateRound > -1)
      {
        DateTime dateTime = DateTime::new().AddYears(this.game.Data.StartData.Year - 1).AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1));
        object LoopForResult;
        object CounterResult;
        if (ObjectFlowControl.ForLoopControl.ForLoopInitObj(Counter,  2,  round,  1,  LoopForResult,  CounterResult))
        {
          do
          {
            if (this.game.Data.AlternateRound == 31)
            {
              dateTime = dateTime.AddMonths(1);
            }
            else
            {
              TimeSpan timeSpan = new TimeSpan(this.game.Data.AlternateRound, 0, 0, 0);
              dateTime = dateTime.Add(timeSpan);
            }
          }
          while (ObjectFlowControl.ForLoopControl.ForNextCheckObj(CounterResult, LoopForResult,  CounterResult));
        }
        str = Strings.Trim(Conversion.Str( dateTime.Day)) + "/" + Strings.Trim(Conversion.Str( dateTime.Month));
      }
      else if (this.game.Data.AlternateRound2 > -1)
      {
        DateTime dateTime = DateTime::new().AddYears(this.game.Data.StartData.Year - 1).AddMonths(this.game.Data.StartData.Month - 1).AddDays( (this.game.Data.StartData.Day - 1)).AddHours( this.game.Data.StartData.Hour);
        object LoopForResult;
        object CounterResult;
        if (ObjectFlowControl.ForLoopControl.ForLoopInitObj(Counter,  2,  round,  1,  LoopForResult,  CounterResult))
        {
          do
          {
            TimeSpan timeSpan = new TimeSpan(0, this.game.Data.AlternateRound2, 0, 0);
            dateTime = dateTime.Add(timeSpan);
          }
          while (ObjectFlowControl.ForLoopControl.ForNextCheckObj(CounterResult, LoopForResult,  CounterResult));
        }
        if (((this.game.Data.Round <= 10 ? 1 : 0) & 0) != 0)
          str = Strings.Trim(Conversion.Str( dateTime.Day)) + "/" + Strings.Trim(Conversion.Str( dateTime.Month)) + " " + Strings.Trim(Conversion.Str( dateTime.Hour)) + ":00";
        else
          str = Strings.Trim(Conversion.Str( dateTime.Day)) + "/" + Strings.Trim(Conversion.Str( dateTime.Month)) + " " + Strings.Trim(Conversion.Str( dateTime.Hour)) + "h".to_owned();
      }
      else
        str = Strings.Trim(Conversion.Str( round));
      return str;
    }
  }
}
