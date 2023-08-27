// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleByteCache
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Imaging;
// usingSystem.Runtime.InteropServices;

namespace WindowsApplication1
{
  pub class SimpleByteCache
  {
    pub byte[,] cacheBig;
    pub byte[,] cacheMed;
    pub byte[,] cacheSmall;
    pub cacheTotal: i32;
    pub byte[] singleCacheBig;
    pub byte[] singleCacheMed;
    pub byte[] singleCacheSmall;
    pub byte[,] singleFredCacheBig;
    pub byte[,] singleFredCacheMed;
    pub byte[,] singleFredCacheSmall;

    pub SimpleByteCache()
    {
      self.cacheBig = new byte[1, 1];
      self.cacheMed = new byte[1, 1];
      self.cacheSmall = new byte[1, 1];
      self.singleFredCacheBig = new byte[65, 1];
      self.singleFredCacheMed = new byte[65, 1];
      self.singleFredCacheSmall = new byte[65, 1];
    }

    pub void SetSingleFredAlphaCache(
       bmp: Bitmap,
       bmpBig: Bitmap,
       bmpSmall: Bitmap,
      bitmapNr: i32)
    {
      s: String = BitmapStore.tmpFileName[bitmapNr];
      if (Information.IsNothing( bmp))
      {
        bmp = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + s);
        bmp.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        str1: String = BitmapStore.MakeBigString(s);
        bmpBig = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + str1);
        bmpBig.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        str2: String = BitmapStore.MakeSmallString(s);
        bmpSmall = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + str2);
        bmpSmall.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      self.singleFredCacheMed = new byte[65, 3072];
      let mut index1: i32 = 1;
      IntPtr scan0;
      do
      {
        let mut x1: i32 = DrawMod.TGame.SHEETX[index1] * 64;
        let mut y1: i32 = DrawMod.TGame.SHEETY[index1] * 48;
        let mut index2: i32 = 0;
        if (DrawMod.TGame.Data.Product >= 6)
        {
          let mut width: i32 = 64;
          let mut height: i32 = 48;
          let mut num1: i32 = x1 * 4;
          let mut num2: i32 = (x1 + width) * 4 - num1;
          Rectangle rect = Rectangle::new(x1, y1, width, height);
          BitmapData bitmapdata = bmp.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
          byte[] destination = new byte[num2 - 1 + 1];
          let mut num3: i32 = bitmapdata.Height - 1;
          for (let mut index3: i32 = 0; index3 <= num3; index3 += 1)
          {
            scan0 = bitmapdata.Scan0;
            Marshal.Copy((IntPtr) (scan0.ToInt64() + (long) (index3 * bitmapdata.Stride)), destination, 0, destination.Length);
            let mut num4: i32 = num2 - 1;
            for (let mut index4: i32 = 3; index4 <= num4; index4 += 4)
            {
              self.singleFredCacheMed[index1, index2] = destination[index4];
              index2 += 1;
            }
          }
          bmp.UnlockBits(bitmapdata);
        }
        else
        {
          let mut num5: i32 = y1;
          let mut num6: i32 = y1 + 47;
          for (let mut y2: i32 = num5; y2 <= num6; y2 += 1)
          {
            let mut num7: i32 = x1;
            let mut num8: i32 = x1 + 63;
            for (let mut x2: i32 = num7; x2 <= num8; x2 += 1)
            {
              pixel: Color = bmp.GetPixel(x2, y2);
              self.singleFredCacheMed[index1, index2] = pixel.A;
              index2 += 1;
            }
          }
        }
        index1 += 1;
      }
      while (index1 <= 64);
      self.singleFredCacheBig = new byte[65, 12288];
      let mut index5: i32 = 1;
      do
      {
        let mut x3: i32 = DrawMod.TGame.SHEETX[index5] * 128;
        let mut y3: i32 = DrawMod.TGame.SHEETY[index5] * 96;
        let mut index6: i32 = 0;
        if (DrawMod.TGame.Data.Product >= 6)
        {
          let mut width: i32 = 128;
          let mut height: i32 = 96;
          let mut num9: i32 = x3 * 4;
          let mut num10: i32 = (x3 + width) * 4 - num9;
          Rectangle rect = Rectangle::new(x3, y3, width, height);
          BitmapData bitmapdata = bmpBig.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
          byte[] destination = new byte[num10 - 1 + 1];
          let mut num11: i32 = bitmapdata.Height - 1;
          for (let mut index7: i32 = 0; index7 <= num11; index7 += 1)
          {
            scan0 = bitmapdata.Scan0;
            Marshal.Copy((IntPtr) (scan0.ToInt64() + (long) (index7 * bitmapdata.Stride)), destination, 0, destination.Length);
            let mut num12: i32 = num10 - 1;
            for (let mut index8: i32 = 3; index8 <= num12; index8 += 4)
            {
              self.singleFredCacheBig[index5, index6] = destination[index8];
              index6 += 1;
            }
          }
          bmpBig.UnlockBits(bitmapdata);
        }
        else
        {
          let mut num13: i32 = y3;
          let mut num14: i32 = y3 + 95;
          for (let mut y4: i32 = num13; y4 <= num14; y4 += 1)
          {
            let mut num15: i32 = x3;
            let mut num16: i32 = x3 +  sbyte.MaxValue;
            for (let mut x4: i32 = num15; x4 <= num16; x4 += 1)
            {
              pixel: Color = bmpBig.GetPixel(x4, y4);
              self.singleFredCacheBig[index5, index6] = pixel.A;
              index6 += 1;
            }
          }
        }
        index5 += 1;
      }
      while (index5 <= 64);
      self.singleFredCacheSmall = new byte[65, 768];
      let mut index9: i32 = 1;
      do
      {
        let mut x5: i32 = DrawMod.TGame.SHEETX[index9] * 32;
        let mut y5: i32 = DrawMod.TGame.SHEETY[index9] * 24;
        let mut index10: i32 = 0;
        if (DrawMod.TGame.Data.Product >= 6)
        {
          let mut width: i32 = 32;
          let mut height: i32 = 24;
          let mut num17: i32 = x5 * 4;
          let mut num18: i32 = (x5 + width) * 4 - num17;
          Rectangle rect = Rectangle::new(x5, y5, width, height);
          BitmapData bitmapdata = bmpSmall.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
          byte[] destination = new byte[num18 - 1 + 1];
          let mut num19: i32 = bitmapdata.Height - 1;
          for (let mut index11: i32 = 0; index11 <= num19; index11 += 1)
          {
            scan0 = bitmapdata.Scan0;
            Marshal.Copy((IntPtr) (scan0.ToInt64() + (long) (index11 * bitmapdata.Stride)), destination, 0, destination.Length);
            let mut num20: i32 = num18 - 1;
            for (let mut index12: i32 = 3; index12 <= num20; index12 += 4)
            {
              self.singleFredCacheSmall[index9, index10] = destination[index12];
              index10 += 1;
            }
          }
          bmpSmall.UnlockBits(bitmapdata);
        }
        else
        {
          let mut num21: i32 = y5;
          let mut num22: i32 = y5 + 23;
          for (let mut y6: i32 = num21; y6 <= num22; y6 += 1)
          {
            let mut num23: i32 = x5;
            let mut num24: i32 = x5 + 31;
            for (let mut x6: i32 = num23; x6 <= num24; x6 += 1)
            {
              pixel: Color = bmpSmall.GetPixel(x6, y6);
              self.singleFredCacheSmall[index9, index10] = pixel.A;
              index10 += 1;
            }
          }
        }
        index9 += 1;
      }
      while (index9 <= 64);
      BitmapStore.Dispose(bitmapNr);
    }

    pub fn SetSingleAlphaCache( bmp: Bitmap,  bmpBig: Bitmap,  bmpSmall: Bitmap)
    {
      self.singleCacheMed = new byte[bmp.Width * bmp.Height - 1 + 1];
      let mut index1: i32 = 0;
      let mut num1: i32 = 0;
      let mut num2: i32 = 0;
      let mut num3: i32 = num2;
      let mut num4: i32 = num2 + 47;
      for (let mut y: i32 = num3; y <= num4; y += 1)
      {
        let mut num5: i32 = num1;
        let mut num6: i32 = num1 + 63;
        for (let mut x: i32 = num5; x <= num6; x += 1)
        {
          pixel: Color = bmp.GetPixel(x, y);
          self.singleCacheMed[index1] = pixel.A;
          index1 += 1;
        }
      }
      self.singleCacheSmall = new byte[bmpSmall.Width * bmpSmall.Height - 1 + 1];
      let mut index2: i32 = 0;
      let mut num7: i32 = num2;
      let mut num8: i32 = num2 + 23;
      for (let mut y: i32 = num7; y <= num8; y += 1)
      {
        let mut num9: i32 = num1;
        let mut num10: i32 = num1 + 31;
        for (let mut x: i32 = num9; x <= num10; x += 1)
        {
          pixel: Color = bmpSmall.GetPixel(x, y);
          self.singleCacheSmall[index2] = pixel.A;
          index2 += 1;
        }
      }
      self.singleCacheBig = new byte[bmpBig.Width * bmpBig.Height - 1 + 1];
      let mut index3: i32 = 0;
      let mut num11: i32 = num2;
      let mut num12: i32 = num2 + 95;
      for (let mut y: i32 = num11; y <= num12; y += 1)
      {
        let mut num13: i32 = num1;
        let mut num14: i32 = num1 +  sbyte.MaxValue;
        for (let mut x: i32 = num13; x <= num14; x += 1)
        {
          pixel: Color = bmpBig.GetPixel(x, y);
          self.singleCacheBig[index3] = pixel.A;
          index3 += 1;
        }
      }
    }

    pub void SetMultiRGBCache(
       bmp: Bitmap,
       bmpBig: Bitmap,
       bmpSmall: Bitmap,
      bitmapNr: i32)
    {
      s: String = BitmapStore.tmpFileName[bitmapNr];
      width1: i32;
      try
      {
        width1 = bmp.Width;
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        bmp = (Bitmap) null;
        ProjectData.ClearProjectError();
      }
      if (Information.IsNothing( bmp))
      {
        bmp = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + s);
        bmp.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        str1: String = BitmapStore.MakeBigString(s);
        bmpBig = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + str1);
        bmpBig.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        str2: String = BitmapStore.MakeSmallString(s);
        bmpSmall = BitmapStore.LoadBitmap(BitmapStore.GraphicsPath + str2);
        bmpSmall.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        if ( DrawMod.TGame.Data.RuleVar[451] > 0.0 & DrawMod.TGame.Data.Product >= 7)
        {
          let mut stringListById: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID( Math.Round( DrawMod.TGame.Data.RuleVar[451]));
          let mut length: i32 = DrawMod.TGame.Data.StringListObj[stringListById].Length;
          for (let mut index: i32 = 0; index <= length; index += 1)
          {
            if ( Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 0])) > 0)
            {
              String2: String = DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 1];
              let mut fr: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 2]));
              let mut fg: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 3]));
              let mut fb: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 4]));
              let mut tr: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 5]));
              let mut tg: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 6]));
              let mut tb: i32 =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[stringListById].Data[index, 7]));
              if (Strings.InStr(Strings.LCase(s.Replace("\\", "/").Replace("//", "/")), String2) > 0)
              {
                DrawMod.ModifyColorOfBitmapHighSpeed( bmp, fr, fg, fb, tr, tg, tb);
                DrawMod.ModifyColorOfBitmapHighSpeed( bmpBig, fr, fg, fb, tr, tg, tb);
                DrawMod.ModifyColorOfBitmapHighSpeed( bmpSmall, fr, fg, fb, tr, tg, tb);
              }
            }
          }
        }
      }
      let mut width2: i32 = bmp.Width;
      let mut height1: i32 = bmp.Height;
      num1: i32;
      if (width2 == 424)
        num1 = 8;
      self.cacheTotal = num1;
      self.cacheMed = new byte[num1 * num1 + 1, 12288];
      let mut num2: i32 = 0;
      let mut num3: i32 = num1 - 1;
      pixel: Color;
      for (let mut index1: i32 = 0; index1 <= num3; index1 += 1)
      {
        let mut num4: i32 = num1 - 1;
        for (let mut index2: i32 = 0; index2 <= num4; index2 += 1)
        {
          let mut x1: i32 = index1 * 53 - 11;
          let mut y1: i32 = index2 * 48;
          if ((index1 + 2) % 2 > 0)
            y1 += 24;
          let mut index3: i32 = 0;
          if (x1 >= 0 & y1 + 48 <= bmp.Height & DrawMod.TGame.Data.Product >= 6)
          {
            let mut width3: i32 = 64;
            let mut height2: i32 = 48;
            let mut num5: i32 = x1 * 4;
            let mut num6: i32 = (x1 + width3) * 4 - num5;
            Rectangle rect = Rectangle::new(x1, y1, width3, height2);
            BitmapData bitmapdata = bmp.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
            byte[] destination = new byte[num6 - 1 + 1];
            let mut num7: i32 = bitmapdata.Height - 1;
            for (let mut index4: i32 = 0; index4 <= num7; index4 += 1)
            {
              Marshal.Copy((IntPtr) (bitmapdata.Scan0.ToInt64() + (long) (index4 * bitmapdata.Stride)), destination, 0, destination.Length);
              let mut num8: i32 = num6 - 1;
              for (let mut index5: i32 = 0; index5 <= num8; index5 += 1)
                self.cacheMed[index1 * 8 + index2, index3 + index5] = destination[index5];
              index3 += num6;
            }
            bmp.UnlockBits(bitmapdata);
          }
          else
          {
            let mut num9: i32 = y1;
            let mut num10: i32 = y1 + 47;
            for (let mut index6: i32 = num9; index6 <= num10; index6 += 1)
            {
              let mut num11: i32 = x1;
              let mut num12: i32 = x1 + 63;
              for (let mut index7: i32 = num11; index7 <= num12; index7 += 1)
              {
                let mut x2: i32 = index7;
                let mut y2: i32 = index6;
                if (x2 < 0)
                  x2 = bmp.Width + index7;
                if (y2 > bmp.Height - 1)
                  y2 -= bmp.Height;
                pixel = bmp.GetPixel(x2, y2);
                self.cacheMed[index1 * 8 + index2, index3] = pixel.B;
                self.cacheMed[index1 * 8 + index2, index3 + 1] = pixel.G;
                self.cacheMed[index1 * 8 + index2, index3 + 2] = pixel.R;
                self.cacheMed[index1 * 8 + index2, index3 + 3] = pixel.A;
                index3 += 4;
              }
            }
          }
        }
      }
      width1 = bmpBig.Width;
      height1 = bmpBig.Height;
      self.cacheBig = new byte[num1 * num1 + 1, 49152];
      num2 = 0;
      let mut num13: i32 = num1 - 1;
      for (let mut index8: i32 = 0; index8 <= num13; index8 += 1)
      {
        let mut num14: i32 = num1 - 1;
        for (let mut index9: i32 = 0; index9 <= num14; index9 += 1)
        {
          let mut x3: i32 = index8 * 106 - 22;
          let mut y3: i32 = index9 * 96;
          if ((index8 + 2) % 2 > 0)
            y3 += 48;
          let mut index10: i32 = 0;
          if (x3 >= 0 & y3 + 96 <= bmpBig.Height & DrawMod.TGame.Data.Product >= 6)
          {
            let mut width4: i32 = 128;
            let mut height3: i32 = 96;
            let mut num15: i32 = x3 * 4;
            let mut num16: i32 = (x3 + width4) * 4 - num15;
            Rectangle rect = Rectangle::new(x3, y3, width4, height3);
            BitmapData bitmapdata = bmpBig.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
            byte[] destination = new byte[num16 - 1 + 1];
            let mut num17: i32 = bitmapdata.Height - 1;
            for (let mut index11: i32 = 0; index11 <= num17; index11 += 1)
            {
              Marshal.Copy((IntPtr) (bitmapdata.Scan0.ToInt64() + (long) (index11 * bitmapdata.Stride)), destination, 0, destination.Length);
              let mut num18: i32 = num16 - 1;
              for (let mut index12: i32 = 0; index12 <= num18; index12 += 1)
                self.cacheBig[index8 * 8 + index9, index10 + index12] = destination[index12];
              index10 += num16;
            }
            bmpBig.UnlockBits(bitmapdata);
          }
          else
          {
            let mut num19: i32 = y3;
            let mut num20: i32 = y3 + 95;
            for (let mut index13: i32 = num19; index13 <= num20; index13 += 1)
            {
              let mut num21: i32 = x3;
              let mut num22: i32 = x3 +  sbyte.MaxValue;
              for (let mut index14: i32 = num21; index14 <= num22; index14 += 1)
              {
                let mut x4: i32 = index14;
                let mut y4: i32 = index13;
                if (x4 < 0)
                  x4 = bmpBig.Width + index14;
                if (y4 > bmpBig.Height - 1)
                  y4 -= bmpBig.Height;
                pixel = bmpBig.GetPixel(x4, y4);
                self.cacheBig[index8 * 8 + index9, index10] = pixel.B;
                self.cacheBig[index8 * 8 + index9, index10 + 1] = pixel.G;
                self.cacheBig[index8 * 8 + index9, index10 + 2] = pixel.R;
                self.cacheBig[index8 * 8 + index9, index10 + 3] = pixel.A;
                index10 += 4;
              }
            }
          }
        }
      }
      width1 = bmpSmall.Width;
      height1 = bmpSmall.Height;
      self.cacheSmall = new byte[num1 * num1 + 1, 3072];
      num2 = 0;
      let mut num23: i32 = num1 - 1;
      for (let mut index15: i32 = 0; index15 <= num23; index15 += 1)
      {
        let mut num24: i32 = num1 - 1;
        for (let mut index16: i32 = 0; index16 <= num24; index16 += 1)
        {
          let mut x5: i32 = index15 * 27 - 5;
          let mut y5: i32 = index16 * 24;
          if ((index15 + 2) % 2 > 0)
            y5 += 12;
          let mut index17: i32 = 0;
          if (x5 >= 0 & y5 + 32 <= bmpSmall.Height & DrawMod.TGame.Data.Product >= 6)
          {
            let mut width5: i32 = 32;
            let mut height4: i32 = 24;
            let mut num25: i32 = x5 * 4;
            let mut num26: i32 = (x5 + width5) * 4 - num25;
            Rectangle rect = Rectangle::new(x5, y5, width5, height4);
            BitmapData bitmapdata = bmpSmall.LockBits(rect, ImageLockMode.ReadOnly, PixelFormat.Format32bppPArgb);
            byte[] destination = new byte[num26 - 1 + 1];
            let mut num27: i32 = bitmapdata.Height - 1;
            for (let mut index18: i32 = 0; index18 <= num27; index18 += 1)
            {
              Marshal.Copy((IntPtr) (bitmapdata.Scan0.ToInt64() + (long) (index18 * bitmapdata.Stride)), destination, 0, destination.Length);
              let mut num28: i32 = num26 - 1;
              for (let mut index19: i32 = 0; index19 <= num28; index19 += 1)
                self.cacheSmall[index15 * 8 + index16, index17 + index19] = destination[index19];
              index17 += num26;
            }
            bmpSmall.UnlockBits(bitmapdata);
          }
          else
          {
            let mut num29: i32 = y5;
            let mut num30: i32 = y5 + 23;
            for (let mut index20: i32 = num29; index20 <= num30; index20 += 1)
            {
              let mut num31: i32 = x5;
              let mut num32: i32 = x5 + 31;
              for (let mut index21: i32 = num31; index21 <= num32; index21 += 1)
              {
                let mut x6: i32 = index21;
                let mut y6: i32 = index20;
                if (x6 < 0)
                  x6 = bmpSmall.Width + index21;
                if (y6 > bmpSmall.Height - 1)
                  y6 -= bmpSmall.Height;
                pixel = bmpSmall.GetPixel(x6, y6);
                self.cacheSmall[index15 * 8 + index16, index17] = pixel.B;
                self.cacheSmall[index15 * 8 + index16, index17 + 1] = pixel.G;
                self.cacheSmall[index15 * 8 + index16, index17 + 2] = pixel.R;
                self.cacheSmall[index15 * 8 + index16, index17 + 3] = pixel.A;
                index17 += 4;
              }
            }
          }
        }
      }
      BitmapStore.Dispose(bitmapNr);
    }
  }
}
