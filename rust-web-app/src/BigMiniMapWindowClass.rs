// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BigMiniMapWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

namespace WindowsApplication1
{
  pub class BigMiniMapWindowClass : WindowClass
  {
     int MapId;
     int minheight;
     int minwidth;
     bool hidden;

    pub BigMiniMapWindowClass(ref tGame: GameClass, let mut tminheight: i32 =  0, let mut tminwidth: i32 =  200)
      : base(ref tGame, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight)
    {
      this.minheight = tminheight;
      this.minwidth = tminwidth;
      this.hidden = false;
      let mut tsubpart: SubPartClass =  new MiniMapPartClass(tGame, tx: (tGame.ScreenWidth - tminwidth), ty: (tGame.ScreenHeight - tminheight), talsounits: true, trealhex: true);
      this.MapId = this.AddSubPart(ref tsubpart, 0, 0, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight, 0);
    }

    pub fn DoRefresh()
    {
      this.SubPartList[this.SubpartNr(this.MapId)].Paint();
      this.PaintCurrentBitmap(this.MapId);
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index] && this.SubPartID[index] == this.MapId)
          {
            let mut selectX: i32 =  this.game.SelectX;
            let mut selectY: i32 =  this.game.SelectY;
            this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
            windowReturnClass.AddCommand(4, 39);
            windowReturnClass.AddCommand(4, 53);
            if (b == 2)
            {
              this.hidden = false;
              if (this.MapId > 0)
                this.RemoveSubPart(this.MapId);
              let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, this.hidden, DrawMod.TGame.ScreenWidth - this.minwidth, DrawMod.TGame.ScreenHeight - this.minheight, true, true);
              this.MapId = this.AddSubPart(ref tsubpart, 0, 0, DrawMod.TGame.ScreenWidth - this.minwidth, DrawMod.TGame.ScreenHeight - this.minheight, 0);
            }
            if (b == 1)
            {
              if (!this.hidden)
                this.hidden = true;
              if (this.MapId > 0)
                this.RemoveSubPart(this.MapId);
              let mut tsubpart: SubPartClass =  new MiniMapPartClass(DrawMod.TGame, this.hidden, DrawMod.TGame.ScreenWidth - this.minwidth, DrawMod.TGame.ScreenHeight - this.minheight, true, true);
              this.MapId = this.AddSubPart(ref tsubpart, 0, 0, DrawMod.TGame.ScreenWidth - this.minwidth, DrawMod.TGame.ScreenHeight - this.minheight, 0);
            }
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
