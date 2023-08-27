// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CombatStatusWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class CombatStatusWindowClass : WindowClass
  {
     Info1Id: i32;
     info2id: i32;

    pub CombatStatusWindowClass(ref tGame: GameClass)
      : base(ref tGame, 200, tGame.ScreenHeight)
    {
      this.dostuff();
    }

    pub fn DoRefresh() => this.dostuff();

    pub fn dostuff()
    {
      if (this.Info1Id > 0)
        this.RemoveSubPart(this.Info1Id);
      if (this.info2id > 0)
        this.RemoveSubPart(this.info2id);
      let mut tsubpart: SubPartClass =  TextPartClass::new("Status", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
      this.Info1Id = this.AddSubPart(ref tsubpart, 10, 2, 200, 20, 0);
      if (this.game.TempCombat.CombatType == 1)
      {
        tsubpart =  TextPartClass::new("Land Attack", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 2)
      {
        tsubpart =  TextPartClass::new("Sea Attack", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 3)
      {
        tsubpart =  TextPartClass::new("Land Art Attack", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 4)
      {
        tsubpart =  TextPartClass::new("Sea Art Attack", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 5)
      {
        tsubpart =  TextPartClass::new("Airstrike", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 6)
      {
        tsubpart =  TextPartClass::new("Bombing Run", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 9)
      {
        tsubpart =  TextPartClass::new("Paradrop", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else if (this.game.TempCombat.CombatType == 10)
      {
        tsubpart =  TextPartClass::new("Amphibious Attack", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
      else
      {
        if (this.game.TempCombat.CombatType != 11)
          return;
        tsubpart =  TextPartClass::new("Ambush / LandSurprise", Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 200, 20, false);
        this.info2id = this.AddSubPart(ref tsubpart, 10, 42, 200, 20, 0);
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 =  this.SubPartID[index];
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
